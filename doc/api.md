# 数字京师API

> 🚩 施工中

## 参考

### 北京师范大学校园网IP地址范围
[校内ip位置查询](https://ip.bnu.edu.cn)
 
- 教育网IP地址：
   ```
   202.112.80.0/20
   219.224.16.0/20
   210.31.64.0/20
   59.64.32.0/19
   ```
- 联通公网IP地址：
   ```
   114.255.218.0/23
   114.255.220.0/24
   ```
- 电信公网IP地址：
   ```
   219.142.121.0/26
   219.142.99.0/27
   219.143.237.96/27
   219.143.237.128/25
   60.247.18.0/24
   ```
- 校内私网地址：
   ```
   172.16.0.0/16
   172.21.0.0/16
   172.22.0.0/16
   172.23.0.0/16
   172.29.0.0/16
   172.24.0.0/16
   172.25.0.0/16
   ```


## 登录

关键请求如下，需要先GET请求一次挂上`JSESSIONID`和获得`lt`值，然后再POST表单登录

```python

import requests

cookies = {
    'JSESSIONID': '上次请求获得',
    'cas_hash': '',
    'Language': 'zh_CN',
}

headers = {
    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7',
    'Accept-Language': 'zh,en-US;q=0.9,en;q=0.8,zh-CN;q=0.7',
    'Cache-Control': 'max-age=0',
    'Connection': 'keep-alive',
    'Content-Type': 'application/x-www-form-urlencoded',
    'DNT': '1',
    'Origin': 'https://cas.bnu.edu.cn',
    'Referer': 'https://cas.bnu.edu.cn/cas/login?service=https%3A%2F%2Fone.bnu.edu.cn%2Fdcp%2F',
    'Sec-Fetch-Dest': 'document',
    'Sec-Fetch-Mode': 'navigate',
    'Sec-Fetch-Site': 'same-origin',
    'Sec-Fetch-User': '?1',
    'Upgrade-Insecure-Requests': '1',
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0',
    'sec-ch-ua': '"Chromium";v="130", "Microsoft Edge";v="130", "Not?A_Brand";v="99"',
    'sec-ch-ua-mobile': '?0',
    'sec-ch-ua-platform': '"Windows"',
}

params = {
    'service': 'https://one.bnu.edu.cn/dcp/',
}

data = {
    'rsa': '用户名密码的res加密',
    'ul': '学号长度，加密时用到',
    'pl': '密码长度，加密时用到',
    'lt': '上次请求获得',
    'execution': 'e1s1',
    '_eventId': 'submit',
}

response = requests.post('https://cas.bnu.edu.cn/cas/login', params=params, cookies=cookies, headers=headers, data=data)


```

登录js：[文件](https://cas.bnu.edu.cn/cas/comm/js/login9.js)

加密js： [文件](https://cas.bnu.edu.cn/cas/comm/js/des.js)

登录成功会302重定向到目标网址（service=xxxx，默认`https://one.bnu.edu.cn/dcp` ),成功标志是Set-Cookies里面会设置`CASTGC`，用于授权各服务 ，登录失败是重新回本页面

## vpn登录

## 教务初始化

## 教务系统数据接口

