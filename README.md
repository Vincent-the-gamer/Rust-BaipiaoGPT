# Rust-BaipiaoGPT
Free chatting with gpt-3.5-turbo.

## Use with frontend
You can use this server with my web page.

Link: [https://github.com/Vincent-the-gamer/BaipiaoGPT-WebUI](https://github.com/Vincent-the-gamer/BaipiaoGPT-WebUI)

Clone this repo and modify `utils/axios.ts`,
change `baseURL` to your server host and port.

~~~typescript
// 有上下文的后端接口
export const axiosWithContext: AxiosInstance = axios.create({
    baseURL: "http://localhost:8081"
})
~~~


## Features
* multi-platform
* chat with context
* dynamic port binding
* cross origin

## Usage

### Run with default port: 
Default Port: 8080
~~~shell
# darwin/linux
./rust-baipiaogpt  # use your file name!!!!

# windows
rust-baipiaogpt.exe  # use your file name!!!!
~~~

### Run with custom port: 
~~~shell
# darwin/linux
./rust-baipiaogpt 2333 # use your file name!!!!

# windows
rust-baipiaogpt.exe 2333 # use your file name!!!!
~~~


### APIs

| URL               | Method   | Description                                              |     
| :---------------- | -------- | -------------------------------------------------------- |
| /chat             | post     | Return the response of AI, save contexts in an array     | 
| /clearContext     | get      | clear context                                            | 
| /showContextCount | get      | get count of current contexts                            | 
| /regenerate       | get      | remove latest question and answer, re-ask the latest question to get new answer | 


## Build
Rust enviroment required.

Install Rust: [https://www.rust-lang.org/](https://www.rust-lang.org/)

~~~shell
git clone https://github.com/Vincent-the-gamer/Rust-BaipiaoGPT.git
cargo run # debug
cargo build # test build
cargo build --release # release
~~~