# 北师小鸦3

![License](https://img.shields.io/badge/license-MIT-blue)
[![Tauri](https://img.shields.io/badge/Tauri-2.0.0-blue?logo=tauri)]()
[![Vue.js](https://img.shields.io/badge/vue.js-v3-green?logo=vue.js)](https://github.com/vuejs/vue-next)
[![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Android](https://img.shields.io/badge/android-supported-yellow)](https://www.android.com/)
![Platform](https://img.shields.io/badge/platform-Android%20|%20iOS-orange)

北师小鸦3是一个专为北京师范大学（BNU）学生设计的校园生活助手应用程序。这个应用程序在其前身[北师小鸦2](https://github.com/BNU-Crow/BNU-Xiaoya)的基础上，为BNU学生提供了更加完善和流畅的体验。
## 功能特点 
> 照抄北师小鸦的 🤣
- **数字京师一键登录**：得益于我们强大的后端支持，即使在高峰时段也能享受高成功率和平滑的登录体验。
- **轻松查询成绩和GPA**：使用多种计算方法和详细的学分模块快速查看你的成绩。
- **便捷的考试安排获取**：与教务处同步，及时了解考试安排和倒计时提醒，保持备考状态。
- **快速访问京师体育**：轻松追踪你的体育活动和出勤情况。
- **高效的空教室查询**：轻松找到空闲教室，节省你的时间。
- **全面的图书馆服务**：轻松访问图书馆资源和研究论文。
- **新生小贴士**：包含校历、校园地图和新生必备的联系方式。

## 计划中的功能
- **自动选/抢课功能**：灵感来源于 [BNUCourseGetter](https://github.com/LeafYeeXYZ/BNUCourseGetter)。
- **体育馆抢场**：思路来自 [仓库](https://github.com/MosRat/BnuBadmintonBook)
- 欢迎提出其他idea

## 技术细节
- **前端**：使用[VUE 3](https://vuejs.org/)构建。
- **核心**：使用[Rust](https://www.rust-lang.org/)开发。
- **平台**：目前支持Android构建；计划支持iOS（我没有Mac，我打算买一台）。
- **数字京师API**：细节请查看 <a href='./doc/api.md'>api.md</a>
- 
## 构建
要在Android设备上构建BnuCrow3，请在安装了[Tauri](https://tauri.app/start/)和[Rust](https://www.rust-lang.org/tools/install)的Windows设备上按照以下步骤操作：
1. 克隆仓库：
   ```bash
   git clone https://github.com/MosRat/BnuCrow3.git
   ```
2. 进入项目目录：
   ```bash
   cd BNUCrow3
   ```
3. 初始化项目：
   ```bash
   pnpm install
   tauri android init
   ```
4. 在模拟器或手机上调试此应用程序（确保已连接adb）：
   ```bash
   tauri android dev
   ```

5. 使用tauri构建项目：
   ```bash
   tauri android build -t aarch64 --apk
   ```
## 许可证
此项目采用MIT许可证。详情请参阅[LICENSE](LICENSE_MIT)文件。

## 致谢
特别感谢原[北师小鸦2团队](https://github.com/BNU-Crow)及其贡献者们的启发和基础工作。
