#![allow(dead_code, unused_imports)]
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Cursor, Write};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use tauri_plugin_http::reqwest::{redirect::Policy, header, ClientBuilder, Client, Proxy};
use anyhow::{anyhow, Result};
use chrono::Local;
use regex::Regex;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use tauri::Manager;
use url::form_urlencoded;
use image::{DynamicImage, GenericImageView};
use dddd_ocr::Rec;

use super::session::Session;
use super::encrypt::{enc_params, str_enc};
use super::{APP, REC};


const CHECK_ENTRY: &str = "https://webvpn.bnu.edu.cn/login";
const INNER_CAS: &str = "https://cas.bnu.edu.cn/cas/login";
const INTER_CAS: &str = "https://onevpn.bnu.edu.cn/http/77726476706e69737468656265737421f3f652d2253e7d1e7b0c9ce29b5b/cas/login?service=https%3A%2F%2Fonevpn.bnu.edu.cn%2Flogin%3Fcas_login%3Dtrue";
const INNER_DCP: &str = "https://one.bnu.edu.cn/dcp/forward.action?path=/portal/portal&p=home";
const INTER_DCP: &str = "https://onevpn.bnu.edu.cn/https/77726476706e69737468656265737421fff944d2253e7d1e7b0c9ce29b5b/dcp/forward.action?path=/portal/portal&p=home";
const INNER_AAM: &str = "http://zyfw.bnu.edu.cn";
const INTER_AAM: &str = "https://onevpn.bnu.edu.cn/http/77726476706e69737468656265737421eaee478b69326645300d8db9d6562d";
const INNER_AAM_PUB: &str = "http://zyfw.prsc.bnu.edu.cn";
const INTER_AAM_PUB: &str = "https://onevpn.bnu.edu.cn/http/77726476706e69737468656265737421eaee478b69207a437d468ba28d1b26314f9ba11c";
const INNER_PROXY: &str = "https://proxy.bnu.edu.cn";
const INTER_PROXY: &str = "https://onevpn.bnu.edu.cn/https/77726476706e69737468656265737421e0e54e843e7e6a5e6b468ca88d1b203b";

const INNER_CARD: &str = "https://card.bnu.edu.cn";
const INTER_CARD: &str = "https://onevpn.bnu.edu.cn/https/77726476706e69737468656265737421f3f6539869326645300d8db9d6562d";

const INNER_TY: &str = "https://tycg.bnu.edu.cn";
const INTER_TY: &str = "https://webvpn.bnu.edu.cn/https/57787a7876706e323032336b657940246c01011fb2019d5be449fe2ddfb88b";

const WX_CAS: &str = "https://weixin.bnu.edu.cn/login.php";

const DCP_HOST: &str = "one.bnu.edu.cn";
const VPN_HOST: &str = "onevpn.bnu.edu.cn";
const AAM_HOST: &str = "zyfw.bnu.edu.cn";

const AAM_PUB_HOST: &str = "zyfw.prsc.bnu.edu.cn";

const CARD_HOST: &str = "card.bnu.edu.cn";
const TY_HOST: &str = "tycg.bnu.edu.cn";

pub enum TyList {
    YMQ,
    PPQ,
    SWYQ,
    YYG,
}

impl FromStr for TyList {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<TyList, Self::Err> {
        match input.to_lowercase().as_str() {
            "ymq" => Ok(TyList::YMQ),
            "ppq" => Ok(TyList::PPQ),
            "swyq" => Ok(TyList::SWYQ),
            "yyg" => Ok(TyList::YYG),
            _ => Err(anyhow!("Invalid input for parse TyList: {}", input)),
        }
    }
}

impl TyList {
    const TIME_KEYS: [&'static str; 14] = [
        "08:00-09:00",
        "09:00-10:00",
        "10:00-11:00",
        "11:00-12:00",
        "12:00-13:00",
        "13:00-14:00",
        "14:00-15:00",
        "15:00-16:00",
        "16:00-17:00",
        "17:00-18:00",
        "18:00-19:00",
        "19:00-20:00",
        "20:00-21:00",
        "21:00-22:00"
    ];
    pub fn get_cgid(&self) -> String {
        match self {
            TyList::YMQ => "6fdb8db0-5b72-4ee9-baba-c1bef6db6b0b",
            TyList::PPQ => "d855c916-26a8-47cc-a6b4-d02ba0c1dd20",
            TyList::SWYQ => "136d9258-31f6-490b-ac15-d1ec5a386a3b",
            TyList::YYG => "917a6cc5-1f9d-4b63-bce7-6fa1afbd72a3",
        }.to_string()
    }

    pub fn get_query(&self) -> String {
        match self {
            TyList::YMQ => "dataOne=%7B%22cgid%22%3A%226fdb8db0-5b72-4ee9-baba-c1bef6db6b0b%22%2C%22xmlx%22%3A%225%22%2C%22yyrq%22%3A%222024-10-26%22%2C%22xydm%22%3A%2211%22%7D",
            TyList::PPQ => "dataOne=%7B%22cgid%22%3A%22d855c916-26a8-47cc-a6b4-d02ba0c1dd20%22%2C%22xmlx%22%3A%224%22%2C%22yyrq%22%3A%222024-10-26%22%2C%22xydm%22%3A%2211%22%7D",
            TyList::SWYQ => "dataOne=%7B%22cgid%22%3A%22136d9258-31f6-490b-ac15-d1ec5a386a3b%22%2C%22xmlx%22%3A%229%22%2C%22yyrq%22%3A%222024-10-26%22%2C%22xydm%22%3A%2211%22%7D",
            TyList::YYG => "dataOne=%7B%22cgid%22%3A%22917a6cc5-1f9d-4b63-bce7-6fa1afbd72a3%22%2C%22xmlx%22%3A%227%22%2C%22yyrq%22%3A%222024-10-26%22%2C%22xydm%22%3A%2211%22%7D",
        }.to_string()
    }

    pub fn get_data(&self, playground: &str, time: &str) -> Result<String> {
        match self.get_details().get(playground) {
            Some(v) => match v.get("sjklist").unwrap().get(time) {
                None => { Err(anyhow!("No match time!")) }
                Some(d) => {
                    Ok(d.to_string())
                }
            }
            None => Err(anyhow!("No match playground!"))
        }
    }

    pub fn get_details(&self) -> Value {
        match self {
            TyList::YMQ => serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/ymq.json"))).unwrap(),
            TyList::PPQ => serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/ppq.json"))).unwrap(),
            TyList::SWYQ => serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/wqc.json"))).unwrap(),
            TyList::YYG => serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/yyg.json"))).unwrap(),
        }
    }

    pub fn get_keys(&self) -> Value {
        match self {
            TyList::YMQ => json!(["羽1", "羽2", "羽3", "羽4", "羽5", "羽6", "羽7", "羽8", "二层东", "二层西", "小综合1", "小综合2", "小综合3", "小综合4"]),
            TyList::PPQ => json!(["乒1", "乒2", "乒3", "乒4", "乒5", "乒6", "乒7", "乒8", "乒9", "乒10", "乒11"]),
            TyList::SWYQ => json!(["网1", "网2", "网3", "网4", "网5"]),
            TyList::YYG => json!([]),
        }
    }

    pub fn parse_request(&self, data: &str, code: &str, datetime: &str) -> String {
        Self::serialize(&match self {
            TyList::YMQ => format!(r#"{{"yycds": [{data}], "cgid": "6fdb8db0-5b72-4ee9-baba-c1bef6db6b0b", "xmlx": "5", "zje": 5, "yyrq": "{datetime}", "txrs": [], "zffs": "1", "authcode": "{code}"}}"#),
            TyList::PPQ => format!(r#"{{"yycds": [{data}], "cgid": "d855c916-26a8-47cc-a6b4-d02ba0c1dd20", "xmlx": "4", "zje": 5, "yyrq": "{datetime}", "txrs": [], "zffs": "1", "authcode": "{code}"}}"#),
            TyList::SWYQ => format!(r#"{{"yycds": [{data}], "cgid": "136d9258-31f6-490b-ac15-d1ec5a386a3b", "xmlx": "9", "zje": 5, "yyrq": "{datetime}", "txrs": [], "zffs": "1", "authcode": "{code}"}}"#),
            TyList::YYG => format!("{data}"),
        })
    }

    fn serialize(s: &str) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("dataOne", s)
            .finish()
            .replace("+", "")
    }
}

impl Display for TyList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_cgid())
    }
}

pub async fn judge_network() -> Result<bool> {
    Ok(ClientBuilder::new()
        .redirect(Policy::none())
        .danger_accept_invalid_certs(true)
        // .proxy(Proxy::all("http://192.168.137.1:10809").unwrap())
        .build()?
        .get(CHECK_ENTRY)
        .send()
        .await?
        .status() == 200)
}

pub fn parse_class_table(html_data: &str) -> Result<Value> {
    use serde_json::{Value, json};
    use regex::Regex;
    use std::collections::HashMap;
    let html_data = html_data
        // .replace("\n","")
        // .replace("\t","")
        ;
    // let mut file = File::create("output_new.html")?;
    // file.write_all(html_data.as_bytes())?;
    let tbody_re = Regex::new(r"(?s)<tbody>(.*?)</tbody>").unwrap();
    let mut courses = Vec::new();

    if let Some(tbody_caps) = tbody_re.captures(&html_data) {
        let tbody_content = &tbody_caps[1];

        let tr_re = regex::Regex::new(r"(?s)<tr>(.*?)</tr>").unwrap();
        for tr_caps in tr_re.captures_iter(tbody_content) {
            let tr_content = &tr_caps[1];

            let td_re = regex::Regex::new(r"<td[^>]*>(.*?)</td>")?;
            let mut data = Vec::new();

            for td_caps in td_re.captures_iter(tr_content) {
                data.push(td_caps[1].trim().to_string());
            }

            if let Some(full_course_name) = data.get(0) {
                // Extract course name
                let course_name_re = regex::Regex::new(r"](.*)")?;
                let course_name = course_name_re.captures(full_course_name)
                    .and_then(|caps| caps.get(1))
                    .map_or("", |m| m.as_str());
                match data.get(8) {
                    Some(exempt)  if exempt == "否" =>
                        { // Extract teacher and schedule
                            let empty = "".to_string();
                            let teacher = data.get(4).unwrap_or(&empty);
                            let schedule = data.get(5).unwrap_or(&empty);

                            // Add to courses list
                            courses.push(json!({
                                "course_name": course_name,
                                "teacher": teacher,
                                "schedule": schedule,
                            }));
                        }
                    _ => {}
                }
            }
        }
    }

    let mut schedule_by_week: HashMap<u32, Vec<Value>> = HashMap::new();

    let re = Regex::new(r"(\d+)(?:-(\d+))?周(?:\(([单双])\))? (\S)\[(\d+)-(\d+)] (\S+)\((\d+)\)").unwrap();

    let day_map = vec!["一", "二", "三", "四", "五", "六", "日"];

    for course in courses {
        let course_name = course["course_name"].as_str().unwrap();
        let teacher = course["teacher"].as_str().unwrap();
        let schedule = course["schedule"].as_str().unwrap();

        for cap in re.captures_iter(schedule) {
            let start_week: u32 = cap[1].parse().unwrap();
            let end_week: u32 = cap.get(2).map_or(start_week, |m| m.as_str().parse().unwrap());
            let week_type = cap.get(3).map(|m| m.as_str());
            let day = &cap[4];
            let start_period: u32 = cap[5].parse().unwrap();
            let end_period: u32 = cap[6].parse().unwrap();
            let location = &cap[7];

            for week in start_week..=end_week {
                if (week_type == Some("单") && week % 2 == 0) || (week_type == Some("双") && week % 2 != 0) {
                    continue;
                }

                let weekday = day_map.iter().position(|&d| d == day).map(|d| d + 1).unwrap_or(0);

                let entry = json!({
                    "name": course_name,
                    "teacher": teacher,
                    "location": location,
                    "time": format!("{}-{}", start_period, end_period),
                    "weekday": weekday.to_string(),
                });

                let schedule = schedule_by_week.entry(week).or_insert_with(Vec::new);

                if let Some(existing) = schedule.iter_mut().find(|e| e["name"] == course_name && e["time"] == entry["time"] && e["weekday"] == entry["weekday"]) {
                    let existing_location = existing["location"].as_str().unwrap();
                    let new_location = format!("{}, {}", existing_location, location);
                    existing["location"] = json!(new_location);
                } else {
                    schedule.push(entry);
                }
            }
        }
    }

    // Convert courses list to JSON and print
    let json_data = json!(schedule_by_week);
    Ok(json_data)
}

pub fn parse_exam(html_data: &str) -> Result<Value> {
    let re = Regex::new(r#"<tr.*?><td.*?>(\d+)</td><td.*?>(.*?)</td><td.*?>(.*?)</td><td.*?>(.*?)</td><td.*?>(.*?)</td><td.*?>(.*?)</td><td.*?>(.*?)</td><td.*?>(\d+)</td></tr>"#)
        .unwrap();
    let mut courses = Vec::new();

    for cap in re.captures_iter(html_data) {
        let course = json!({
            "index": cap[1].parse::<u32>()?,
            "course_code": cap[2].to_string(),
            "credits": cap[3].parse::<f32>()?,
            "category": cap[4].to_string(),
            "assessment": cap[5].to_string(),
            "exam_time": cap[6].to_string(),
            "location": cap[7].to_string(),
            "seat_number": cap[8].parse::<u32>()?,
        });
        courses.push(course);
    }
    Ok(json!(courses))
}

pub fn parse_score(html_data: &str) -> Result<Value> {
    let row_re = Regex::new(r"(?s)<tr>(.*?)</tr>").unwrap();
    let cell_re = Regex::new(r"(?s)<td.*?>(.*?)</td>").unwrap();

    let mut courses = Vec::new();

    for row in row_re.captures_iter(html_data) {
        let mut data = Vec::new();
        for cell in cell_re.captures_iter(&row[1]) {
            data.push(cell[1].trim().replace("&nbsp;", ""));
        }

        if data.len() == 12 {
            let course = Score {
                semester: data[0].clone(),
                course: data[1].clone(),
                credits: data[2].parse().unwrap_or(0.0),
                category: data[3].clone(),
                course_type: data[4].clone(),
                assessment_method: data[5].clone(),
                study_type: data[6].clone(),
                usual_score: data[7].parse().ok(),
                final_score: data[8].parse().ok(),
                total_score: data[9].clone(),
                minor_mark: data[10].clone(),
                remarks: data[11].clone(),
            };
            courses.push(course);
        }
    }
    Ok(json!(courses))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Soap {
    soap: f32,
    balance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exam {
    index: u32,
    course_code: String,
    credits: f32,
    category: String,
    assessment: String,
    exam_time: String,
    location: String,
    seat_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Score {
    semester: String,
    course: String,
    credits: f32,
    category: String,
    course_type: String,
    assessment_method: String,
    study_type: String,
    usual_score: Option<f32>,
    final_score: Option<f32>,
    total_score: String,
    minor_mark: String,
    remarks: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ElectiveCourseForm {
    xktype: String,      // 选课类型
    xn: String,          // 学年
    xq: String,          // 学期
    xh: String,          // 学号
    nj: String,          // 年级
    zydm: String,        // 专业代码
    kcdm: String,        // 课程代码
    kclb1: String,       // 课程类别1
    kclb2: String,       // 课程类别2
    kclb3: String,       // 课程类别3
    khfs: String,        // 考核方式
    skbjdm: String,      // 上课班级代码
    skbzdm: String,      // 上课班组代码
    xf: String,          // 学分
    is_check_Time: String, // 是否检查时间
    kknj: String,        // 开课年级
    kkzydm: String,      // 开课专业代码
    kcfw: String,        // 课程范围
}

impl ElectiveCourseForm {
    fn new(mut data: serde_json::Value) -> Self {
        // 检查并添加缺失字段
        let keys = [
            "xktype", "xn", "xq", "xh", "nj", "zydm", "kcdm", "kclb1", "kclb2", "kclb3",
            "khfs", "skbjdm", "skbzdm", "xf", "is_check_Time", "kknj", "kkzydm", "kcfw"
        ];

        for key in &keys {
            if !data.get(key).is_some() {
                data[key] = Value::String("".to_string());
            }
        }
        let mut form: ElectiveCourseForm = serde_json::from_value(data).unwrap();

        if form.kcfw.is_empty() {
            form.kcfw = "zxbnj".to_string();
        }

        form
    }

    fn encode(&self) -> Result<String> {
        eprintln!("{self:#?}");
        Ok(serde_urlencoded::to_string(&self)?)
    }
}


#[derive(Debug, Clone)]
pub struct DcpSession {
    session: Session,
    pub inner: bool,
    pub info: Option<HashMap<String, String>>,
}

impl Deref for DcpSession {
    type Target = Session;
    fn deref(&self) -> &Session {
        &self.session
    }
}

impl DerefMut for DcpSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.session
    }
}

impl DcpSession {
    pub fn build(
        // username: &str,
        // password: &str,
    ) -> Result<Self> {
        // let inner = judge_network()
        //     .await?;
        let mut err: Option<String> = None;
        let inner = true;
        // eprintln!("login cookies: {}", std::fs::read_to_string( APP
        //                 .get()
        //                 .map(|app| {
        //                     app.path()
        //                         .app_data_dir()
        //                         .unwrap()
        //                         .join("cookies.json")
        //                 }).unwrap_or("cookies.json".parse().unwrap())
        // ).unwrap_or("no cookie".to_string()));
        let session = Session::try_new(
            APP
                .get()
                .map(|app| {
                    app.path()
                        .app_data_dir()
                        .unwrap()
                        .join("cookies.json")
                }).unwrap_or("cookies.json".parse().unwrap())
        )?;
        let mut dcp = DcpSession { session, inner, info: Some(HashMap::new()) };
        Ok(dcp)
        // eprintln!("!alive {}",dcp.check_session_live().await?);
        // if !dcp.check_session_live().await? {
        //     eprintln!("recolonising..");
        //     err = dcp.session_login(username, password).await?;
        //     eprintln!("{:?}", err);
        // }
        // if let Some(ref mut info) = dcp.info {
        //     info.insert("username".to_string(), username.to_string());
        //     info.insert("password".to_string(), password.to_string());
        // }
        //
        //
        // if dcp.check_session_live().await?
        // {
        //     dcp.save_cookie();
        //     Ok(dcp)
        // } else {
        //     Err(anyhow::anyhow!("login fail due to {:?} live: {} cookie {:?}!",
        //         err,
        //         dcp.check_session_live().await?,
        //         dcp.session.state.cookie_store)
        //     )
        // }
    }

    pub async fn test_build(username: &str,
                            password: &str) -> Result<Self> {
        let mut s = DcpSession::build()?;
        s.try_login(username, password).await?;
        Ok(s)
    }

    pub fn get_info(&self) -> Option<&HashMap<String, String>> {
        self.info.as_ref()
    }

    pub fn get_info_mut(&mut self) -> Option<&mut HashMap<String, String>> {
        self.info.as_mut()
    }

    pub async fn set_inner(&mut self) -> Result<bool> {
        self.inner = !judge_network().await?;
        Ok(self.inner)
    }

    pub async fn try_login(&mut self,
                           username: &str,
                           password: &str,
    ) -> Result<String> {
        self.set_inner().await?;
        eprintln!("{username} / {password} | inner =  {}", self.inner);

        let mut err = None;
        if !self.check_session_live().await? {
            eprintln!("recolonising..");
            err = self.session_login(username, password).await?;
            eprintln!("{:?}", err);
        }
        if let Some(ref mut info) = self.info {
            info.insert("username".to_string(), username.to_string());
            info.insert("password".to_string(), password.to_string());
        }


        if self.check_session_live().await?
        {
            self.save_cookie();
            Ok(String::from("successfully login!"))
        } else {
            self.clear_cookie()?;
            eprintln!("{:?}", self.session.state.cookie_store.lock().unwrap());
            Err(
                anyhow::anyhow!("login fail due to {:?} live: {} !",
                err,
                self.check_session_live().await?,
                )
            )
        }
    }
    pub async fn session_login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<String>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
        headers.insert("Referer", (if self.inner { INNER_CAS } else { INTER_CAS }).parse().unwrap());
        headers.insert("Host", (if self.inner { DCP_HOST } else { VPN_HOST }).parse().unwrap());

        if self.inner {
            self.session.get(INNER_CAS)
                .headers(headers.clone())
                .send()
                .await?;
        }

        let res = self.session.get(if self.inner { INNER_CAS } else { INTER_CAS })
            .headers(headers.clone())
            .send()
            .await?
            .text()
            .await?;
        let re_lt = regex::Regex::new(r#"input.*?lt.*?value="(.*?)""#)?;
        let re_ex = regex::Regex::new(r#"name="execution" value="(.*?)""#)?;
        let re_err = regex::Regex::new(r#"class="login_box_notice1">(.*?)<"#)?;
        let lt = re_lt.captures(&res)
            .ok_or(anyhow!("cant reg for lt value"))?
            .get(1)
            .unwrap()
            .as_str();
        let ex = re_ex.captures(&res)
            .ok_or(anyhow!("cant reg for ex value"))?
            .get(1)
            .unwrap()
            .as_str();
        let params = [
            ("rsa", str_enc(&format!("{}{}{}", username, password, lt), "1", "2", "3")),
            ("ul", username.len().to_string()),
            ("pl", password.len().to_string()),
            ("lt", String::from(lt)),
            ("execution", String::from(ex)),
            ("_eventId", "submit".to_string())
        ];

        let _res = self.session.post(if self.inner { INNER_CAS } else { INTER_CAS })
            .headers(headers.clone())
            .form(&params)
            .send()
            .await?
            .text()
            .await?
            ;
        // eprintln!("login res : {}", _res);
        let msg = re_err.captures(&_res)
            // .ok_or(anyhow!("cant reg for error msg"))?
            .map(|m| m.get(1).unwrap().as_str().to_string());


        // eprintln!("after post {:?}",_res.text().await?);
        // if msg.is_none() {
        //     msg= Some(_res);
        // }

        Ok(msg)
    }
    async fn check_session_live(&self) -> Result<bool> {
        Ok(self.session
            .get(if self.inner { INNER_DCP } else { INTER_DCP })
            .send()
            .await?
            .text().await?.contains("Digitalized DCP")
        )
    }

    pub async fn get_soap_info(&self) -> Result<Soap> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0".parse().unwrap());


        let res = self.session.get((if self.inner { INNER_PROXY } else { INTER_PROXY }).to_string() + "/proxy/businessServlet?jsonpCallback=jsonp1728561748976&project=bnu&type=soap&method=getUserInfo&id_number=" + {
            &self.info.as_ref().unwrap()["username"]
        })
            .headers(headers)
            .send().await?
            .text().await?;
        let re_usage = regex::Regex::new(r#""sum_bytes":(\d+)"#)?;
        let re_balance = regex::Regex::new(r#""user_balance":(.*?),"#)?;
        Ok(
            Soap {
                soap: re_usage.captures(&res)
                    .ok_or(anyhow!("cant reg for usage"))?
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<f32>()? / 1e9f32,
                balance: re_balance.captures(&res)
                    .ok_or(anyhow!("cant reg for balance"))?
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<f32>()?,
            }
        )
    }

    pub async fn get_avatar(&self) -> Result<String> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Referer", (if self.inner { INNER_DCP } else { INTER_DCP }).parse().unwrap());
        headers.insert("Host", (if self.inner { DCP_HOST } else { VPN_HOST }).parse().unwrap());
        headers.insert("Content-Type", "text/plain;charset=UTF-8".parse().unwrap());
        headers.insert("clientType", "json".parse().unwrap());
        headers.insert("render", "json".parse().unwrap());
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0".parse().unwrap());

        let res: Value = self.session.post((if self.inner { INNER_DCP } else { INTER_DCP })
            .strip_suffix("/dcp/forward.action?path=/portal/portal&p=home")
            .unwrap()
            .to_string()
            + "/dcp/profile/profile.action")
            .headers(headers.clone())
            .body("{\"map\":{\"method\":\"getAvatar\",\"params\":null},\"javaClass\":\"java.util.HashMap\"}")
            .send().await?
            .json().await?;

        let image = self.session.get((if self.inner { INNER_DCP } else { INTER_DCP })
            .strip_suffix("/dcp/forward.action?path=/portal/portal&p=home")
            .unwrap()
            .to_string()
            + "/dcp/uploadfiles/avatar/large/"
            + res["list"][0]["map"]["AVATAR_L_ID"]
            .as_str()
            .ok_or(anyhow!("cant get avatar path in {}",res))?
        ).headers(headers.clone())
            .send().await?
            .bytes().await?;
        let path: PathBuf =
            // "test.png".into();
            APP
                .get()
                .unwrap()
                .path()
                .app_data_dir()?
                .join(format!("{}.jpg", self.get_info().unwrap()["username"]));
        eprintln!("save avatar at {path:?}");

        File::create(&path)?
            .write_all(&image)?;
        Ok(path.to_str().unwrap().to_string())
    }

    pub async fn init_aam(&mut self) -> Result<()> {
        if let Some(ref mut info) = self.info {
            if info.get("aam").is_none() {
                let mut headers = header::HeaderMap::new();
                headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
                headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse().unwrap());
                headers.insert("Accept-Language", "en".parse().unwrap());
                headers.insert("DNT", "1".parse().unwrap());
                headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());

                eprintln!("aam url:{:?}", if self.inner { INNER_AAM } else { INTER_AAM });
                let res = self.session.get(if self.inner { INNER_AAM } else { INTER_AAM })
                    .headers(headers)
                    .send().await?
                    ;


                if res.url()
                    .as_str()
                    .contains("homes.html") {
                    info.insert("aam".into(), "1".into());
                } else {
                    return Err(anyhow!("Error init aam with url {:?}",res.url().as_str()));
                }
            }
        } else {
            return Err(anyhow!("Cant open info dict!"));
        }

        let (xn, xq, xqm) = self.get_year_term().await?;
        if let Some(ref mut info) = self.info {
            info.insert("xn".to_string(), xn.to_string()); // 学年 2024
            info.insert("xq".to_string(), xq.to_string()); // 学期  秋季学期
            info.insert("xqm".to_string(), xqm); // 学期码 0：秋季 1：春季
        } else {
            return Err(anyhow!("Cant open info dict!"));
        }

        let (week, date) = self.get_week_date().await?;
        if let Some(ref mut info) = self.info {
            info.insert("week".to_string(), week.to_string()); // 周数 6
            info.insert("date".to_string(), date.to_string()); // 日期 27
        } else {
            return Err(anyhow!("Cant open info dict!"));
        }
        let res = self.get_base_info().await?;
        if let Some(ref mut info) = self.info {
            let fields = vec![
                "bjmc", // 班级名称 人工智能2021
                "xm", // 姓名 张三
                "yxb", // 院学部 人工智能学院
                "zymc", // 专业名称 人工智能
                "yhxh", // 学号 20221115xxxx
                "xb", // 性别 男
                "xh" //内部学号 2021xxxxxxxx
            ];
            for field in fields {
                let re = regex::Regex::new(&format!(r"<{0}>(.*?)</{0}>", field))?;
                if let Some(captures) = re.captures(&res) {
                    info.insert(field.to_string(), captures[1].to_string());
                }
            }
        } else {
            return Err(anyhow!("Cant open info dict!"));
        }

        Ok(())
    }

    pub async fn get_base_info(&self) -> Result<String> {
        if let Some(ref info) = self.info {
            if !info.get("aam").is_none() {
                let mut headers = header::HeaderMap::new();
                headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
                headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
                headers.insert("Referer", (if self.inner { INNER_AAM } else { INTER_AAM }).parse().unwrap());
                headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
                let client = &self.session;
                let res = client.get((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
                    "/STU_BaseInfoAction.do?hidOption=InitData&menucode_current=JW13020101")
                    .headers(headers)
                    .send()
                    .await?
                    .text()
                    .await?;

                // eprintln!("{}", &res);
                Ok(res)
            } else {
                Err(anyhow!("aam not init!"))
            }
        } else {
            Err(anyhow!("Cant find aam in info!"))
        }
    }

    pub async fn get_class_table(&self) -> Result<String> {
        if let Some(ref info) = self.info {
            if !info.get("aam").is_none() {
                let mut headers = header::HeaderMap::new();
                headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
                headers.insert("Referer", ((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() + "/student/xkjg.wdkb.jsp?menucode=JW130418").parse().unwrap());
                headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
                let client = &self.session;
                let res = client.get((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
                    "/wsxk/xkjg.ckdgxsxdkchj_data10319.jsp?params=eG49MjAyNCZ4cT0w")
                    .headers(headers)
                    .send()
                    .await?
                    .text()
                    .await?;
                // eprintln!("{}", &res);
                Ok(res)
            } else {
                Err(anyhow!("aam not init!"))
            }
        } else {
            Err(anyhow!("Cant find aam in info!"))
        }
    }

    pub async fn get_exam(&self, xn: u32, xq: u32, kslc: u32) -> Result<String> {
        if let Some(ref info) = self.info {
            if !info.get("aam").is_none() {
                let mut headers = header::HeaderMap::new();
                headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
                headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
                headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
                headers.insert("Referer", ((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() + "/student/ksap.ksapb.html?menucode=JW130603&v=99836172930629611391601").parse().unwrap());
                headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
                let client = &self.session;
                let res = client.post((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
                    "/taglib/DataTable.jsp?tableId=2538")
                    .headers(headers)
                    .body(format!("xh=&xn={xn}&xq={xq}&kslc={kslc}&xnxqkslc={xn}%2C{xq}%2C{kslc}&menucode_current=JW130603"))
                    .send()
                    .await?
                    .text()
                    .await?;
                // eprintln!("{}", &res);
                Ok(res)
            } else {
                Err(anyhow!("aam not init!"))
            }
        } else {
            Err(anyhow!("Cant find aam in info!"))
        }
    }

    pub async fn get_score(&self, xn: u32, xq: u32) -> Result<String> {
        if let Some(ref info) = self.info {
            if !info.get("aam").is_none() {
                let mut headers = header::HeaderMap::new();
                headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
                headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
                headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
                headers.insert("Referer", ((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() + "/student/xscj.stuckcj.jsp?menucode=JW130706").parse().unwrap());
                headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
                let client = &self.session;
                let token = client.post((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
                    "/frame/menus/js/SetTokenkey.jsp")
                    .headers(headers)
                    .body("menucode=xscj.stuckcj.my.jsp")
                    .send().await?
                    .text().await?;


                let mut headers = header::HeaderMap::new();
                headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
                headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
                headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
                headers.insert("Referer", ((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() + "/student/xscj.stuckcj.jsp?menucode=JW130706&v=42763172931060547849095").parse().unwrap());
                headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
                let client = &self.session;
                let res = client.post((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
                    "/student/xscj.stuckcj_data.jsp")
                    .headers(headers)
                    .body(format!("sjxz=sjxz3&ysyx=yscj&zfx=0&t={token}&xn={}&xn1={}&xq={}&ysyxS=on&sjxzS=on&zfxS=on&menucode_current=JW130706",
                                  xn, xn + 1, xq
                    ))
                    .send()
                    .await?
                    .text()
                    .await?;
                // eprintln!("{}", &res);
                Ok(res)
            } else {
                Err(anyhow!("aam not init!"))
            }
        } else {
            Err(anyhow!("Cant find aam in info!"))
        }
    }

    pub async fn get_year_term(&self) -> Result<(i64, i64, String)> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36".parse().unwrap());
        headers.insert("Host", (if self.inner { AAM_PUB_HOST } else { VPN_HOST }).parse().unwrap());


        let res: Value = self.session.get((if self.inner { INNER_AAM_PUB } else { INTER_AAM_PUB }).to_string() + "/jw/common/showYearTerm.action")
            .headers(headers)
            .send().await?
            .json().await?;
        // println!("{}", res);
        // Ok((1,1,"2".to_string()))

        Ok((res.get("xn").unwrap().as_str().unwrap().parse().unwrap(),
            res.get("xqM").unwrap().as_str().unwrap().parse().unwrap(),
            res.get("xqName").unwrap().as_str().unwrap().to_string()
        )
        )
    }

    pub async fn get_week_date(&self) -> Result<(i64, i64)> {
        if let Some(ref info) = self.info {
            let mut headers = header::HeaderMap::new();
            headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36".parse().unwrap());
            headers.insert("Host", (if self.inner { AAM_PUB_HOST } else { VPN_HOST }).parse().unwrap());
            headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

            let res: Vec<i64> = self.session.post((if self.inner { INNER_AAM_PUB } else { INTER_AAM_PUB }).to_string() + "/public/getTeachingWeekByDate.action")
                .headers(headers)
                .body(format!("xn={}&xq_m={}&hidOption=getWeek&hdrq={}",
                              info["xn"],
                              info["xq"],
                              Local::now().format("%Y-%m-%d").to_string()))
                .send().await?
                .text().await?
                .split("@")
                .map(|val| val.parse::<i64>().unwrap())
                .collect()
                ;
            // println!("{}", res);
            // Ok((1,1,"2".to_string()))

            Ok((res[0], res[1]))
            // Ok((1,2))
        } else {
            Err(anyhow!("Cant open info dict!"))
        }
    }

    pub async fn get_drop_list(&self) -> Result<Value> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36".parse().unwrap());
        headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

        let res: Value = self.session.post((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
            "/frame/droplist/getDropLists.action")
            .headers(headers)
            .body("comboBoxName=Ms_KBBP_FBXQLLJXAP")
            .send().await?
            .json().await?;
        Ok(res)
    }

    pub async fn get_aam_token_key(&self, menucode: &str) -> Result<String> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36".parse().unwrap());
        headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
        headers.insert("Referer", ((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() + "/student/wsxk.zx10139.jsp").parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());

        let res = self.session.post((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
            "/frame/menus/js/SetTokenkey.jsp")
            .headers(headers)
            .body(format!("menucode={menucode}"))
            .send().await?
            .text().await?;
        Ok(res)
    }

    pub async fn get_aam_encrypt_key(&self) -> Result<(String, String)> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36".parse().unwrap());
        headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
        // headers.insert("Referer", ((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() + "/student/wsxk.zx10139.jsp").parse().unwrap());

        let res = self.session.get((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
            "/custom/js/SetKingoEncypt.jsp")
            .headers(headers)
            .send().await?
            .text().await?;

        let key_re = Regex::new(r"(?s)_deskey = '(.*?)'.*_nowtime = '(.*?)'").unwrap();
        if let Some(captures) = key_re.captures(&res) {
            Ok((
                captures.get(1).unwrap().as_str().to_string(),
                captures.get(2).unwrap().as_str().to_string()
            ))
        } else {
            Err(anyhow!("cant get des key!"))
        }
    }


    pub async fn select_course_zx(&self, course: Value) -> Result<String> {
        let c = course.clone();
        let token = self.get_aam_token_key("wsxk.zx10139.jsp").await?;
        let (key, nowtime) = self.get_aam_encrypt_key().await?;
        let p = ElectiveCourseForm::new(course).encode()? + "&xk_points=0&is_buy_book=0&is_cx=0&is_yxtj=1&menucode_current=JW130403";
        let params = enc_params(&key, &nowtime, &p);
        eprintln!("{c}\n{token}\n{key}\n{nowtime}\n{p}\n{params}");

        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36".parse().unwrap());
        headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
        headers.insert("Accept-Language", "zh,en-US;q=0.9,en;q=0.8,zh-CN;q=0.7".parse().unwrap());
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
        headers.insert("Referer", ((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() + &format!("/student/report/wsxk.zx_promt.jsp?{}", "params=")).parse().unwrap());
        headers.insert("Accept", "text/plain, */*; q=0.01".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        headers.insert("DNT", "1".parse().unwrap());
        // headers.insert("Accept-Encoding", "gzip, deflate".parse().unwrap());
        headers.insert("Origin", (if self.inner { INNER_AAM } else { INTER_AAM }).to_string().parse().unwrap());
        headers.insert("Proxy-Connection", "keep-alive".parse().unwrap());

        let res = self.session.post((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
            "/jw/common/saveElectiveCourseZx.action")
            .headers(headers)
            .body(format!("{params}&timestamp={nowtime}&tokens={token}"))
            //             .body(r#"params=MDAyOTk2MkFCNDQzMUVGRTBGMDMwNzMzRUMzMURGQTkyQTk5NDBFODExMUNEMjgwMDcyQTcwNkNCOEVCQzg2M0U5MjVERUY0OUFGRDYzODRBQ0IzNDI5NzIyM0VBMDc4MDVFQzY3NjJDMzBERDcwRUI3RjdFQzJFRjQzOERENzM2RkE5RjM0RkNGMUI4MDBCQUQyMUE5RTFDNjEyQUU3MTA1RUM2NzYyQzMwREQ3MEVEQkM0NTNCN0Q2NkU4QTEwMjFDRjhCODlDMUJBMkFBRkUzN0ZCOEJENUVEQ0RGQTk2QjMyNzBEQURDMzlDREQxNzFCNDkzMzRDRUVBRjUyMzhCRDg0NkRFOURDMjgzRUVBRkYyOUZBRTM1REVGRjU2NDFENzAxQzM3Nzc2MDk2NTFDMzE4RThDQUQyMDcxRjU4NkFDOEUzQkMyQTJCMDA1QjA2MTA4OTBFNUUzMDRFRkZERjJDODhGRkQ1QjE3Q0JFODlDMkZFRjk4NTAyOThDNjIzOUJFMDQ0RjZDNkYzQjg3MUQ0RDI5RDZFQUQ1MzE2MjM5QkUwNDRGNkM2RjNCRjM5RTBGQkY3MDA5N0RCNDRBOEFEODlCOTJDQTI0OUI2MDY1REQ0MjkyRjJFMTAwMURCMzk2Q0M2QzRGNTMwNTdFM0E4QTNFQUIyNzFCNjkyQ0QxMkFDRDZDQjRFMTJGMDJCRDdFMkMzQUQ1OEVEQUE4REU1NUY4QUE5M0M1RkIwODgyRjBGQ0JDMkFDRjIzMzhBOUVBQjhBNUJENzBBQ0UxREE4QkJEMDhEM0MxMkRCMTEzMEUxQzVGODZDQzdENEEyM0I3M0RGOTNBMjc0OTkyN0U1M0Q2NjA3NDkzOEQxQUI0QkY0QkI1OEE1RTNERUU5MTczMkQ4MEZGN0FBMDQ5QTcyRDU2RTMwRUNBRERBQzZDODAxQTJENDU4QkJENjU3NDc3QkUyOTBEQzk0Mjk5QkE4OTJBQTQwNEM5M0JDODc1QjU0N0NDNDJDNTkwOTY2NjUxRjVCMDc2QkM3MUE5MUE4MURDREZFMTEyNzhDMzY3M0E2RDEzMDFEREY2REEzREM5ODNGQjE3QUIyM0E3MzJBRkY5ODUzNUEyNUUyODEzNzNBQjM5RUQ1RjMwQTczMkFGRjk4NTM1QTI1RTYzMzBCMkVGRjAzRkZFNzZBMTRCRTg1NjQ3MUQzQTRFNTYwRDcxMjBCNkNDQTVDNkM0NkEyQjE2OTdCMUNFMjQyMjI4RUI5NzBGQ0VDQjU5QzE2RTk0MDI5MzBFNTRFOTVCODdGMjY5NDRFMjJGOUEzRTZDRDE1Nzk3NDdCRTgyNEQ2NENERjc4M0ZCMTU4RTU1Njc5NTVFN0Q3QjU4MEI0RkRGMEUxRDZERTkyNkIyQzhENDRFRTlDMDNGNjRDRg==&token=46f3730533cdb70f571cc6c588272307&timestamp=2024-10-25 21:07:37&tokens=
            //
            //
            // MjA1NDIxQ0MyNzExNzhGNkRBQjk2QjQyNzg4QkI5MDI2MUMyRjE0NkIyMzk3MjhDRUVBNTM2Qzg1
            // OTNCMzMwQzEzNTNDNjQyNjExQjM5NTZBQkQ1OUI3MTBEQzdBQTQ3"#)
            .send().await?
            .text().await?;
        Ok(res)
    }

    pub async fn select_course_gx(&self, course: Value) -> Result<String> {
        let c = course.clone();
        let token = self.get_aam_token_key("wsxk.bykxk.jsp").await?;
        let (key, nowtime) = self.get_aam_encrypt_key().await?;
        let p = ElectiveCourseForm::new(course).encode()? + "&kcfw=zxggrx&items=&is_xjls=undefined&btnSubmit=%E6%8F%90%E4%BA%A4&kcmc=&t_skbh=&menucode_current=JW130415";
        let params = enc_params(&key, &nowtime, &p);
        eprintln!("{c}\n{token}\n{key}\n{nowtime}\n{p}\n{params}");

        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36".parse().unwrap());
        headers.insert("Host", (if self.inner { AAM_HOST } else { VPN_HOST }).parse().unwrap());
        headers.insert("Accept-Language", "zh,en-US;q=0.9,en;q=0.8,zh-CN;q=0.7".parse().unwrap());
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
        headers.insert("Referer", ((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() + "/student/wsxk.bykxk.html?menucode=JW130415").parse().unwrap());
        headers.insert("Accept", "text/plain, */*; q=0.01".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        headers.insert("DNT", "1".parse().unwrap());
        // headers.insert("Accept-Encoding", "gzip, deflate".parse().unwrap());
        headers.insert("Origin", (if self.inner { INNER_AAM } else { INTER_AAM }).to_string().parse().unwrap());
        headers.insert("Proxy-Connection", "keep-alive".parse().unwrap());

        let res = self.session.post((if self.inner { INNER_AAM } else { INTER_AAM }).to_string() +
            "/jw/common/saveElectiveCourseGxk.action")
            .headers(headers)
            .body(format!("{params}&timestamp={nowtime}&tokens={token}"))
            //             .body(r#"params=MDAyOTk2MkFCNDQzMUVGRTBGMDMwNzMzRUMzMURGQTkyQTk5NDBFODExMUNEMjgwMDcyQTcwNkNCOEVCQzg2M0U5MjVERUY0OUFGRDYzODRBQ0IzNDI5NzIyM0VBMDc4MDVFQzY3NjJDMzBERDcwRUI3RjdFQzJFRjQzOERENzM2RkE5RjM0RkNGMUI4MDBCQUQyMUE5RTFDNjEyQUU3MTA1RUM2NzYyQzMwREQ3MEVEQkM0NTNCN0Q2NkU4QTEwMjFDRjhCODlDMUJBMkFBRkUzN0ZCOEJENUVEQ0RGQTk2QjMyNzBEQURDMzlDREQxNzFCNDkzMzRDRUVBRjUyMzhCRDg0NkRFOURDMjgzRUVBRkYyOUZBRTM1REVGRjU2NDFENzAxQzM3Nzc2MDk2NTFDMzE4RThDQUQyMDcxRjU4NkFDOEUzQkMyQTJCMDA1QjA2MTA4OTBFNUUzMDRFRkZERjJDODhGRkQ1QjE3Q0JFODlDMkZFRjk4NTAyOThDNjIzOUJFMDQ0RjZDNkYzQjg3MUQ0RDI5RDZFQUQ1MzE2MjM5QkUwNDRGNkM2RjNCRjM5RTBGQkY3MDA5N0RCNDRBOEFEODlCOTJDQTI0OUI2MDY1REQ0MjkyRjJFMTAwMURCMzk2Q0M2QzRGNTMwNTdFM0E4QTNFQUIyNzFCNjkyQ0QxMkFDRDZDQjRFMTJGMDJCRDdFMkMzQUQ1OEVEQUE4REU1NUY4QUE5M0M1RkIwODgyRjBGQ0JDMkFDRjIzMzhBOUVBQjhBNUJENzBBQ0UxREE4QkJEMDhEM0MxMkRCMTEzMEUxQzVGODZDQzdENEEyM0I3M0RGOTNBMjc0OTkyN0U1M0Q2NjA3NDkzOEQxQUI0QkY0QkI1OEE1RTNERUU5MTczMkQ4MEZGN0FBMDQ5QTcyRDU2RTMwRUNBRERBQzZDODAxQTJENDU4QkJENjU3NDc3QkUyOTBEQzk0Mjk5QkE4OTJBQTQwNEM5M0JDODc1QjU0N0NDNDJDNTkwOTY2NjUxRjVCMDc2QkM3MUE5MUE4MURDREZFMTEyNzhDMzY3M0E2RDEzMDFEREY2REEzREM5ODNGQjE3QUIyM0E3MzJBRkY5ODUzNUEyNUUyODEzNzNBQjM5RUQ1RjMwQTczMkFGRjk4NTM1QTI1RTYzMzBCMkVGRjAzRkZFNzZBMTRCRTg1NjQ3MUQzQTRFNTYwRDcxMjBCNkNDQTVDNkM0NkEyQjE2OTdCMUNFMjQyMjI4RUI5NzBGQ0VDQjU5QzE2RTk0MDI5MzBFNTRFOTVCODdGMjY5NDRFMjJGOUEzRTZDRDE1Nzk3NDdCRTgyNEQ2NENERjc4M0ZCMTU4RTU1Njc5NTVFN0Q3QjU4MEI0RkRGMEUxRDZERTkyNkIyQzhENDRFRTlDMDNGNjRDRg==&token=46f3730533cdb70f571cc6c588272307&timestamp=2024-10-25 21:07:37&tokens=
            //
            //
            // MjA1NDIxQ0MyNzExNzhGNkRBQjk2QjQyNzg4QkI5MDI2MUMyRjE0NkIyMzk3MjhDRUVBNTM2Qzg1
            // OTNCMzMwQzEzNTNDNjQyNjExQjM5NTZBQkQ1OUI3MTBEQzdBQTQ3"#)
            .send().await?
            .text().await?;
        Ok(res)
    }

    pub async fn init_card(&mut self) -> Result<()> {
        if let Some(ref mut info) = self.info {
            if info.get("card").is_none() {
                let mut headers = header::HeaderMap::new();
                headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
                headers.insert("Host", (if self.inner { CARD_HOST } else { VPN_HOST }).parse().unwrap());


                let res = self.session.get(if self.inner { INNER_CARD } else { INTER_CARD })
                    .headers(headers)
                    .send().await?
                    ;

                eprintln!("card result:{:?}", res);


                if res.url()
                    .as_str()
                    .contains("plat-pc") {
                    info.insert("card".into(), "1".into());
                    Ok(())
                } else {
                    Err(anyhow!("Error init card with url {:?}",res.text().await?))
                }
            } else { Ok(()) }
        } else {
            Err(anyhow!("Cant open info dict!"))
        }
    }


    pub async fn init_tycg(&self) -> Result<bool> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
        headers.insert("Host", (if self.inner { TY_HOST } else { VPN_HOST }).parse().unwrap());


        let res = self.session.get(if self.inner { INNER_TY } else { INTER_TY })
            .headers(headers)
            .send().await?
            .status().is_success()
            ;

        Ok(res)
    }

    pub async fn get_authcode(&self) -> Result<String> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
        headers.insert("Host", (if self.inner { TY_HOST } else { VPN_HOST }).parse().unwrap());
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0".parse().unwrap());

        let res = self.session.get((if self.inner { INNER_TY } else { INTER_TY }).to_string() + "/core/api/authcode")
            .headers(headers)
            .send().await?
            .bytes().await?;

        let img = Rec::compose_gif(Cursor::new(res))?;
        let code = REC.get().unwrap().predict_str(&img)?;
        Ok(code)
    }

    pub async fn ty_order(&self, proj: &str, datetime: &str, playground: &str, time: &str) -> Result<String> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
        headers.insert("Host", (if self.inner { TY_HOST } else { VPN_HOST }).parse().unwrap());
        headers.insert("Referer", ((if self.inner { INNER_TY } else { INTER_TY }).to_string() + "/www/yy/bg/cgyyDome").parse().unwrap());
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0".parse().unwrap());
        let mut res: Value;
        let proj = TyList::from_str(proj)?;
        let mut count = 0;

        while
        {
            // let s = std::time::Instant::now();
            let code = self.get_authcode().await?;
            let data = proj.parse_request(&proj.get_data(playground, time)?, &code, datetime);
            // eprintln!("[{datetime} {playground} {time}]: get code {:?}", s.elapsed());

            res = self.session.post((if self.inner { INNER_TY } else { INTER_TY }).to_string() + "/www/tycg/yy/appOrder")
                .headers(headers.clone())
                .body(data)
                .send().await?
                .json().await?;
            // eprintln!("[{datetime} {playground} {time}]: send order {:?}", s.elapsed());

            res["message"].as_str().unwrap().contains("验证码")
        } {
            count += 1;
            if count >= 3 { break; }
        }

        Ok(format!("[{datetime} {playground} {time}]: {}", res["message"].as_str().unwrap()))
    }

    // pub async fn get_card_info(&self) -> Result<f32> {
    //     let mut headers = header::HeaderMap::new();
    //     headers.insert("Origin", (if self.inner { INNER_CARD } else { INTER_CARD }).parse().unwrap());
    //     headers.insert("Authorization", "Basic bW9iaWxlX3NlcnZpY2VfcGxhdGZvcm06bW9iaWxlX3NlcnZpY2VfcGxhdGZvcm1fc2VjcmV0".parse().unwrap());
    //     headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    //     headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0".parse().unwrap());
    //
    //     let res = self.session.post((if self.inner { INNER_CARD } else { INTER_CARD }).to_string() + "/berserker-auth/oauth/token")
    //         .headers(headers)
    //         .send().await?
    //         .text().await?;
    //     let re_usage = regex::Regex::new(r#""sum_bytes":(\d+?)"#)?;
    //     let re_balance = regex::Regex::new(r#""user_balance":(.*?),"#)?;
    //     Ok(
    //         (re_usage.captures(&res)
    //              .ok_or(anyhow!("cant reg for usage"))?
    //              .get(1)
    //              .unwrap()
    //              .as_str()
    //              .parse::<f32>()? / 1e9f32,
    //          re_balance.captures(&res)
    //              .ok_or(anyhow!("cant reg for balance"))?
    //              .get(1)
    //              .unwrap()
    //              .as_str()
    //              .parse::<f32>()?
    //         )
    //     )
    // }

}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::sync::Arc;
    use regex::Regex;
    use super::*;
    use tokio;
    use crate::REC;
    // 使用 tokio 运行时


    #[test]
    fn test_parse_ct() {
        use serde_json::{Value, json};
        use regex::Regex;
        use std::collections::HashMap;

        let data = r#"[{"course_name":"形势与政策7","schedule":"3周 日[5-6] 敬文讲堂(400),4-7周 日[5-6] 网上自学(400)","teacher":"王学松"},{"course_name":"操作系统","schedule":"1-8周 一[1-2] 电106(176),1-17周 二[3-4] 电106(176),2-17周 二[7-8] 九302(79),2-17周 二[7-8] 九303(79)","teacher":"肖融"},{"course_name":"运动与体能提升","schedule":"1-4周(单) 六[11-12] 网上自学(400),1-4周(双) 二[9-10] 健美室1(40),5-8周(单) 六[11-12] 网上自学(400),5-8周(双) 二[9-10] 邱季端体武馆-109(30),9-12周(单) 六[11-12] 网上自学(400),9-12周(双) 二[9-10] 健美室1(40),13-17周(单) 六[11-12] 网上自学(400),13-17周(双) 二[9-10] 邱季端体武馆-109(30)","teacher":"杨兆春;张卓;苏荣海;韩青"}]"#;

        let courses: Vec<Value> = serde_json::from_str(data).unwrap();
        let mut schedule_by_week: HashMap<u32, Vec<Value>> = HashMap::new();

        let re = Regex::new(r"(\d+)(?:-(\d+))?周(?:\(([单双])\))? (\S)\[(\d+)-(\d+)] (\S+)\((\d+)\)").unwrap();

        let day_map = vec!["一", "二", "三", "四", "五", "六", "日"];

        for course in courses {
            let course_name = course["course_name"].as_str().unwrap();
            let teacher = course["teacher"].as_str().unwrap();
            let schedule = course["schedule"].as_str().unwrap();

            for cap in re.captures_iter(schedule) {
                let start_week: u32 = cap[1].parse().unwrap();
                let end_week: u32 = cap.get(2).map_or(start_week, |m| m.as_str().parse().unwrap());
                let week_type = cap.get(3).map(|m| m.as_str());
                let day = &cap[4];
                let start_period: u32 = cap[5].parse().unwrap();
                let end_period: u32 = cap[6].parse().unwrap();
                let location = &cap[7];

                for week in start_week..=end_week {
                    if (week_type == Some("单") && week % 2 == 0) || (week_type == Some("双") && week % 2 != 0) {
                        continue;
                    }

                    let weekday = day_map.iter().position(|&d| d == day).map(|d| d + 1).unwrap_or(0);

                    let entry = json!({
                    "name": course_name,
                    "teacher": teacher,
                    "location": location,
                    "time": format!("{}-{}", start_period, end_period),
                    "weekday": weekday.to_string(),
                });

                    let schedule = schedule_by_week.entry(week).or_insert_with(Vec::new);

                    if let Some(existing) = schedule.iter_mut().find(|e| e["name"] == course_name && e["time"] == entry["time"] && e["weekday"] == entry["weekday"]) {
                        let existing_location = existing["location"].as_str().unwrap();
                        let new_location = format!("{}, {}", existing_location, location);
                        existing["location"] = json!(new_location);
                    } else {
                        schedule.push(entry);
                    }
                }
            }
        }

        for week in 1..=20 {
            if let Some(schedule) = schedule_by_week.get(&week) {
                println!("Week {}: {}", week, serde_json::to_string_pretty(schedule).unwrap());
            } else {
                println!("Week {}: No classes", week);
            }
        }
    }
    #[test]
    fn test_enc() {
        eprintln!("{}",
                  str_enc(&format!("{}{}{}", "2021111500367", "a13970549022", "LT-3508148-fC0N7M6cfHUoERdelXgqakEzpaisvr-cas"), "1", "2", "3")
        );
        assert_eq!("B6FC8B80C5445D4B803ADC54DAC99C67F2FD0BC8541EE297C508E7C4F0100F4E1AC49938D444444D7BAB05CC30B82C5D22D3FF327FAAB443EAC0DE5FA8F3AFC53BC886F50E731095B5A9D78235B2302B540185A5BD52847936F21EA674DE3A3672633885147433E6C0594A60977C5A1C70C223E0D986D464FFE60E7080CB764235E2D4A9383432C89A216BCFE6054686",
                   str_enc(&format!("{}{}{}", "2021111500367", "a13970549022", "LT-3508148-fC0N7M6cfHUoERdelXgqakEzpaisvr-cas"), "1", "2", "3")
        )
    }

    #[tokio::test]
    async fn test_judge_network() {
        match judge_network().await {
            Ok(status) => assert!(status, "Network status should be true"),
            Err(e) => panic!("Error occurred: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_dcp_login() {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        // println!("{}", s.get_class_table().await.unwrap());


        // let _res = s.session
        //     .get(INNER_AAM.to_string() + "/frame/homes.html?")
        //     .send()
        //     .await
        //     .unwrap()
        //     ;
        // let mut file = File::create("output_new.html").unwrap();
        // file.write_all(_res.text().await.unwrap().as_bytes()).unwrap();
        // let _res = s.get_class_table().await.unwrap();
        // let _res = s.session
        //     .get(INNER_AAM.to_string() + "/frame/homes.html?")
        //     .send()
        //     .await
        //     .unwrap()
        //     ;
        let mut file = File::create("output_new.html").unwrap();
        file.write_all(s.get_base_info().await.unwrap().as_bytes()).unwrap();
        //
        // let mut headers = header::HeaderMap::new();
        // // headers.insert(header::COOKIE, "Language=zh_CN; JSESSIONID=DB67AB45E4088A5803A879E058F995A6; CASPRIVACY=; CASTGC=TGT-202111150036-136486-nB4gij1r0mYe1ubmuIZNx0mgki0NVVpfDSOr357PbIEtDfGiiq-cas; BIGipServerjioawu=2551847084.36895.0000".parse().unwrap());
        // headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 Safari/537.36".parse().unwrap());
        // headers.insert("Referer", "http://zyfw.bnu.edu.cn".parse().unwrap());
        // headers.insert("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
        // headers.insert("Host", "zyfw.bnu.edu.cn".parse().unwrap());
        //
        //
        // let res = s.post("http://zyfw.bnu.edu.cn/STU_BaseInfoAction.do?hidOption=InitData&menucode_current=JW13020101")
        //     .headers(headers)
        //     .body("")
        //     .send().await.unwrap()
        //     .text().await.unwrap();
        // println!("{}", res);


        s.save_cookie();
        // eprintln!("{:?}", s.get(INNER_AAM.to_string() + "/frame/homes.html?v=65429172845801053408706")
        //     .send()
        //     .await
        //     .unwrap()
        // );
    }

    #[tokio::test]
    async fn test_get_class_table() {
        println!("{:#?}",
                 {
                     let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
                         Ok(s) => s,
                         Err(e) => panic!("{}", e.to_string())
                     };
                     s.init_aam().await.unwrap();
                     eprintln!("{:#?}", s.get_info().unwrap());

                     // println!("{:?}",s.info.as_ref().unwrap());
                     println!("{:#}", parse_class_table(&s.get_class_table().await.unwrap()).unwrap());
                 }
        )
    }

    #[tokio::test]
    async fn test_get_year_term() {
        println!("{:?}",
                 {
                     let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
                         Ok(s) => s,
                         Err(e) => panic!("{}", e.to_string())
                     };
                     s.get_year_term().await.unwrap()
                 }
        )
    }

    #[tokio::test]
    async fn test_get_week_date() {
        println!("{:?}",
                 {
                     let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
                         Ok(s) => s,
                         Err(e) => panic!("{}", e.to_string())
                     };
                     s.init_aam().await.unwrap();
                     s.info.unwrap()
                 }
        )
    }

    #[tokio::test]
    async fn test_get_drop_list() {
        println!("{}",
                 {
                     let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
                         Ok(s) => s,
                         Err(e) => panic!("{}", e.to_string())
                     };
                     s.init_aam().await.unwrap();
                     s.get_drop_list().await.unwrap()
                 }
        )
    }

    #[tokio::test]
    async fn test_get_avatar() {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        eprintln!("{}", (s.get_avatar().await.unwrap()))
    }

    #[tokio::test]
    async fn test_get_soap() -> Result<()> {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        eprintln!("{:?}", (s.get_soap_info().await.unwrap()));


        Ok(())
    }

    #[tokio::test]
    async fn test_get_droplist() -> Result<()> {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        eprintln!("{:#}", (s.get_drop_list().await.unwrap()));


        Ok(())
    }

    #[tokio::test]
    async fn test_get_exam() -> Result<()> {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        let tbody_re = Regex::new(r"(?s)<tbody>(.*?)</tbody>")?;
        eprintln!("{:#}", parse_exam(tbody_re.captures(&s.get_exam(2023, 1, 3).await.unwrap()).unwrap().get(1).unwrap().as_str()).unwrap());
        eprintln!("{:?}", s.get_exam(2024, 1, 1).await.unwrap());


        Ok(())
    }

    #[tokio::test]
    async fn test_get_score() -> Result<()> {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        let tbody_re = Regex::new(r"(?s)<tbody>(.*?)</tbody>")?;
        eprintln!("{:#}", parse_score(tbody_re.captures(&s.get_score(2023, 0).await.unwrap()).unwrap().get(1).unwrap().as_str()).unwrap());
        eprintln!("{:?}", s.get_score(2023, 0, ).await.unwrap());


        Ok(())
    }

    #[tokio::test]
    async fn test_select_course() -> Result<()> {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        eprintln!("{}", s.select_course_gx(
            json!({
    "xktype": "2",
    "xn": "2024",
    "xq": "0",
    "xh": "202161286408",
    "nj": "2021",
    "zydm": "BQ108",
    "kcdm": "2310178582",
    "kclb1": "05",
    "kclb2": "A1",
    "kclb3": "01",
    "khfs": "01",
    "skbjdm": "2310178582-01",
    "skbzdm": "",
    "xf": "1.0",
    "kknj": "",
    "kkzydm": ""
})
        ).await?);


        Ok(())
    }

    #[tokio::test]
    async fn test_get_cookie() -> Result<()> {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        eprintln!("{:}", s.get_cookie().unwrap());
        Ok(())
    }
    #[tokio::test]
    async fn test_get_authcode() -> Result<()> {
        REC.get_or_init(|| Rec::from_embed().unwrap());
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        eprintln!("{:}", s.get_authcode().await.unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn test_ty() -> Result<()> {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        eprintln!("{:}", s.init_tycg().await.unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn test_order_ty() -> Result<()> {
        REC.get_or_init(|| Rec::from_embed().unwrap());

        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };


        s.init_aam().await.unwrap();
        eprintln!("{:}", s.init_tycg().await.unwrap());

        let s = Arc::new(s);
        let s1 = s.clone();
        let s2 = s.clone();
        let s3 = s.clone();

        let a1 =  tokio::spawn(async move {
            let t = std::time::Instant::now();
            let res = s1.ty_order("ymq", "2024-10-29", "羽1", "12:00-13:00").await.unwrap();
            eprintln!("{:?}", t.elapsed());
            res
        }).await?;
        let a2 = tokio::spawn(async move {
            let t = std::time::Instant::now();
            let res = s2.ty_order("ymq", "2024-10-29", "羽2", "12:00-13:00").await.unwrap();
            eprintln!("{:?}", t.elapsed());
            res
        }).await?;
        let a3 =  tokio::spawn(async move {
            let t = std::time::Instant::now();
            let res = s3.ty_order("ymq", "2024-10-29", "羽3", "12:00-13:00").await.unwrap();
            eprintln!("{:?}", t.elapsed());
            res
        }).await?;
        eprintln!("{a1} {a2} {a3}");


        // let results: Vec<_> = vec![
        //     tokio::spawn(async move {
        //         let t = std::time::Instant::now();
        //         let res = s1.ty_order("ymq", "2024-10-29", "羽1", "12:00-13:00").await.unwrap();
        //         eprintln!("{:?}", t.elapsed());
        //         res
        //     }),
        //     tokio::spawn(async move {
        //         let t = std::time::Instant::now();
        //         let res = s2.ty_order("ymq", "2024-10-29", "羽2", "12:00-13:00").await.unwrap();
        //         eprintln!("{:?}", t.elapsed());
        //         res
        //     }),
        //     tokio::spawn(async move {
        //         let t = std::time::Instant::now();
        //         let res = s3.ty_order("ymq", "2024-10-29", "羽3", "12:00-13:00").await.unwrap();
        //         eprintln!("{:?}", t.elapsed());
        //         res
        //     }),
        // ];


        // for result in results.into_iter() {
        //     eprintln!("{}", result.await?);
        // }
        // eprintln!("{:}", s.ty_order("ymq", "2024-10-29", "羽1", "12:00-13:00").await.unwrap());
        // eprintln!("{:}", s.ty_order("ppq", "2024-10-29", "乒1", "12:00-13:00").await.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_parse_request() -> Result<()> {
        let data = TyList::YMQ.get_data("羽1", "12:00-13:00")?;
        // 获取当前日期
        let today = Local::now().naive_local();

        // 计算明天的日期
        let tomorrow = today + chrono::Duration::days(1);

        // 格式化为 YYYY-MM-DD
        let formatted_date = tomorrow.format("%Y-%m-%d").to_string();

        eprintln!("{}", TyList::YMQ.parse_request(&data, "12345", &formatted_date));
        // let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
        //     Ok(s) => s,
        //     Err(e) => panic!("{}", e.to_string())
        // };
        //
        // s.init_aam().await.unwrap();
        // eprintln!("{:}", s.init_tycg().await.unwrap());
        Ok(())
    }


    #[tokio::test]
    async fn test_card_init() {
        let mut s = match DcpSession::test_build("202111150036", "a13970549022").await {
            Ok(s) => s,
            Err(e) => panic!("{}", e.to_string())
        };

        s.init_aam().await.unwrap();
        s.init_card().await.unwrap();
    }
}