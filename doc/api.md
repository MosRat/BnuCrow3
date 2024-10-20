# 数字京师API

> 🚩 施工中

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

登录成功会302重定向到目标网址（service=xxxx），标志是Set-Cookies里面会设置`CASTGC`，默认`https://one.bnu.edu.cn/dcp`，登录失败是重新回本页面

## vpn登录

## 教务初始化

## 教务系统数据接口

