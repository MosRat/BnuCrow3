
<div align="center">
<h1 style="font-size: 40px"> BNU Crow 3 </h1>


![License](https://img.shields.io/badge/license-MIT-blue)
[![Tauri](https://img.shields.io/badge/Tauri-2.0.0-blue?logo=tauri)]()
[![Vue.js](https://img.shields.io/badge/vue.js-v3-green?logo=vue.js)](https://github.com/vuejs/vue-next)
[![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Android](https://img.shields.io/badge/android-supported-yellow)](https://www.android.com/)
![Platform](https://img.shields.io/badge/platform-Android%20|%20iOS-orange)

<h3>English | <a href='./README_zh.md'>中文</a> </h3>
</div>

Welcome to 北师小鸦3, a dedicated campus life assistant app designed for students of Beijing Normal University (BNU). This app builds upon the legacy of its predecessor, [北师小鸦2](https://github.com/BNU-Crow/BNU-Xiaoya), to provide an enhanced and seamless experience for BNU students.


## Features
> Copied from 北师小鸦 🤣
- **Seamless Login to 数字京师**: Enjoy higher success rates and a smoother login experience, even during peak times, thanks to our robust backend.
- **Effortless Grade and GPA Checking**: Quickly access your grades with multiple calculation methods and detailed credit modules.
- **Convenient Exam Schedule Access**: Stay prepared with up-to-date exam arrangements and countdown reminders synced with the academic office.
- **Quick 京师体育 Access**: Easily track your sports activities and attendance.
- **Efficient Classroom Availability**: Save time by finding free classrooms effortlessly.
- **Comprehensive Library Services**: Access library resources and research papers with ease.
- **New Student Tips**: Includes academic calendar, campus map, and essential contact information for newcomers.

## Upcoming Features

- **Automatic Course Enrollment**: Inspired by [BNUCourseGetter](https://github.com/LeafYeeXYZ/BNUCourseGetter).
- **Automated Sports Facility Booking**: Idea from [this repository](https://github.com/MosRat/BnuBadmintonBook)

## Technical Details

- **Frontend**: Built with [Vue 3](https://vuejs.org/).
- **Core**: Developed using [Rust](https://www.rust-lang.org/).
- **Platform**: Currently supports Android builds; iOS support planned (pending access to Mac, I'm planning to buy one).
- **Bnu Web Api**:  See  <a href='./doc/api.md'>api.md</a>

## Build

To build BnuCrow3 for Android device, follow these steps in Windows device with [Tauri](https://tauri.app/start/) and [Rust](https://www.rust-lang.org/tools/install) installed:

1. Clone the repository:
   ```bash
   git clone https://github.com/MosRat/BnuCrow3.git
   ```
2. Navigate to the project directory:
   ```bash
   cd BNUCrow3
   ```
3. Init project:
   ```bash
   pnpm install
   tauri android init
   ```
4. Debug this app on an emulator or phone with (make sure adb is connected) :
   ```bash
   tauri android dev
   ```
   
5. Build the project using tauri:
   ```bash
   tauri android build -t aarch64 --apk
   ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE_MIT) file for details.

## Acknowledgments

Special thanks to the original [北师小鸦2 team](https://github.com/BNU-Crow) and the contributors for their inspiration and foundational work.

