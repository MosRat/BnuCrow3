/*
 * Copyright (c) 2024. MosRat
 * All rights reserved.
 *
 * Project: BnuCrow3
 * File Name: dcp.ts
 * Created Date: 2024/10/30 00:24
 * Author: MosRat (work@whl.moe)
 * Description:
 */

// import {defineStore} from 'pinia'
import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {store} from "@lib/tauri-store";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {ref} from "vue";


export const update = ref(0.0)

// 定义单个课程信息的接口
export interface Course {
    location: string;  // 上课地点
    name: string;      // 课程名称
    teacher: string;   // 教师姓名
    time: string;      // 上课时间
    weekday: string;   // 上课星期
}

// 定义表示整个课程表的类型
export type Schedule = Course[][]

// 会话中的学生信息
export interface Info {
    zymc?: string;      // 专业名称
    password?: string;  // 密码
    yhxh?: string;      // 用户学号
    xn?: string;        // 学年
    xb?: string;        // 性别
    xq?: string;        // 学期
    aam?: string;       // 教务初始化状态
    xqm?: string;       // 学期名称
    xm?: string;        // 姓名
    week?: string;      // 周
    username?: string;  // 用户名
    yxb?: string;       // 院系部
    bjmc?: string;      // 班级名称
    date?: string;      // 日期
    class_table?: Schedule // 课程表
    avatar?: string      // 头像本地文件路径
}

export interface Exam {
    index: number;
    course_code: string;
    credits: number;
    category: string;
    assessment: string;
    exam_time: string;
    location: string;
    seat_number: number;
}

export interface Score {
    semester: string;
    course: string;
    credits: number;
    category: string;
    course_type: string;
    assessment_method: string;
    study_type: string;
    usual_score?: number;
    final_score?: number;
    total_score: string;
    minor_mark: string;
    remarks: string;
}

export interface DcpError {
    error: string,
    file: string,
    line: number
}

function parseScheduleToNestedArray(jsonString: string): Course[][] {
    // 解析 JSON 字符串为对象
    const scheduleObject = JSON.parse(jsonString);

    // 创建一个数组来存储课程信息
    const schedule: Course[][] = [];

    // 遍历对象中的每个键
    for (const key in scheduleObject) {
        if (scheduleObject.hasOwnProperty(key)) {
            // 将字符串键转换为数字索引
            const index = parseInt(key, 10);
            schedule[index] = scheduleObject[key];
        }
    }

    return schedule;
}

/**
 * 对应 Rust 中的 DcpSession
 */
class Dcp {

    login_status: boolean = false // 登录状态
    info?: Info // 基本信息缓存
    class_table?: Course[][] //课程表缓存
    avatar?: string // 头像路径url
    soap?: { soap: string, balance: number } // 网费信息
    /**
     * 登录DcpSession
     * @param username 学号
     * @param password 数字京师密码
     * @return "Successfully login!"
     */
    login = async (username: string, password: string): Promise<string> => {
        const success = await invoke("login", {username, password})
        this.login_status = true
        await getCurrentWindow().listen<null>("info_inited", this.fetch_info)
        return success as string
    }
    /**
     * 获取DcpSession的内外网状态inner
     * @return inner变量
     */
    get_inner = async (): Promise<boolean> => await invoke<boolean>("get_inner")
    /**
     * 获取DcpSession的原始cookies，并格式化为js注入形式
     * ```js
     * document.cookie = 'CASTGC=TGT-202111150036-263465-L5YqHHRgEDToSfosLKb11eAyYr6WL7KsRwASJDIaqme2bggnnn-cas; path=/cas/; domain=cas.bnu.edu.cn; expires="Fri, 31 Dec 9999 23:59:59 GMT";';
     * document.cookie = 'Language=zh_CN; path=/; domain=cas.bnu.edu.cn; expires="Sat, 26 Oct 2024 10:22:13 GMT";';
     * ```
     */
    get_cookie = async (): Promise<string> => await invoke<string>("get_cookie")
    /**
     *  同时获取所有的信息，用作初始化的回调
     */
    fetch_info = () => {
        console.log("fetching data...")
        Promise.all([
            this.get_info().then(this.get_avatar),
            this.get_class_table(),
            this.get_soap_info()
        ]).then(
            () => {
                update.value = Math.random()
                console.log("data updated")
                // console.error(this)
                // console.error(1)
                // console.error(JSON.stringify(this.info))
                console.log(update.value)
            }, err => console.log(err)
        )


    }
    /**
     * 获取基本信息 缓存
     * @return 用户信息结构体
     */
    get_info = async (): Promise<Info> => this.info ?? (this.info = JSON.parse(await invoke("get_base_info")))

    /**
     * 获取课程表 缓存
     */
    get_class_table = async (): Promise<Schedule> => this.class_table ?? (this.class_table = parseScheduleToNestedArray(await invoke("get_class_table")))

    get_avatar = async (): Promise<string> => this.avatar ?? (this.avatar = convertFileSrc((await this.get_info()).avatar as string))

    get_soap_info = async (): Promise<{
        soap: number,
        balance: number
    }> => this.soap ?? (this.soap = await invoke("get_soap_info") as any)

    get_score = async (xn: string, xq: string): Promise<Score[]> => JSON.parse(await invoke<string>("get_score", {
        xn,
        xq
    })) as Score[]
    get_exam = async (xn: string, xq: string): Promise<Exam[]> => JSON.parse(await invoke<string>("get_exam", {
        xn,
        xq
    })) as Exam[]

    webviewTo = async (path: string, js?: string, withCookies?: boolean): Promise<void> => {
        console.log(js || "")
        await invoke("webview_to", {
            path, withCookies: withCookies ?? false, js:
                (js || "") + `
                    window.location.href = "${path}"
                    console.log(window.location.href)
                    // window.location.reload()
                `,
        })
    }

    syncData = async (): Promise<any> => {
        if (JSON.parse(await invoke("get_base_info"))?.aam) {
            this.login_status = true
            this.fetch_info()
        }
    }

    clear_cookies = async (): Promise<any> => await invoke("clear_cookie")

    reset = async () => {
        this.info = undefined
        this.avatar = undefined
        this.class_table = undefined
        this.login_status = false
        await this.clear_cookies()
        await store.reset()
        console.log(this, JSON.stringify(this))
    }
}


const DCP = new Dcp()
// export const useDcp = defineStore('dcp', {
//     state: (): { dcp: Dcp | null } => ({
//         dcp: null,
//     }),
//     actions: {
//         async init() {
//             this.dcp = new Dcp()
//             const curWindow = getCurrentWindow()
//             await curWindow.listen<null>("info_inited", () => {
//                 // @ts-ignore
//                 (this.dcp as Dcp).get_info()
//                 (this.dcp as Dcp).get_class_table()
//                 (this.dcp as Dcp).get_avatar()
//             })
//         }
//     },
// })
export default DCP;