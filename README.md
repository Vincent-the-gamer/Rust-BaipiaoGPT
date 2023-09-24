<p align="center">
    <img src=".github/logo.png" style="height: 80px;"/>
</p>
<h1 align="center">Rust-BaipiaoGPT</h1>
<p align="center">Free chatting with gpt-3.5-turbo.</p>

## Notice
This chat backend is no longer supporting context now due to some reason.

## WebAssembly web page

[https://vincent-the-gamer.github.io/baipiaogpt-wasm-page/](https://vincent-the-gamer.github.io/baipiaogpt-wasm-page/)

### Build webpage from source code
~~~shell
cd webpage
trunk build --release
~~~
Open webpage to chat with AI. 

`WebAssembly` page has no backend server. 

So we can directly deploy it on `GitHub Pages`.
~~~shell
# deploy to GitHub Pages
trunk build --release --public-url /your-github-repo-name/
~~~


## Use core as http service
~~~shell
cd server
cargo build --release
~~~

### Features
* multi-platform
* dynamic port binding
* cross origin
* WebAssembly

### Usage

#### Run at default port: 
Default Port: 8080
~~~shell
# darwin/linux
./rust-baipiaogpt  # use your file name!!!!

# windows
rust-baipiaogpt.exe  # use your file name!!!!
~~~

#### Run at custom port: 
~~~shell
# darwin/linux
./rust-baipiaogpt 2333 # use your file name!!!!

# windows
rust-baipiaogpt.exe 2333 # use your file name!!!!
~~~


#### APIs

| URL               | Method   | Description                                              |     
| :---------------- | -------- | -------------------------------------------------------- |
| /chat             | post     | Return the response of AI, save contexts in an array     | 
| /clearContext     | get      | Clear context                                            | 
| /showContextCount | get      | Get count of current contexts                            | 
| /regenerate       | get      | Remove latest question and answer, re-ask the latest question to get new answer | 


##### API Parameters
Only `/chat` has parameter:

you need to give a request body(json):

~~~typescript
{
    content: "your question"
}

// request example using axios
axios.post("/chat",{
   content: ""
}).then(res => {
    console.log(res.data)
})
~~~

For any other API:
~~~typescript
axios.get("/xxx").then(res => {
    console.log(res.data)
})
~~~

##### API Test
You can use `Postman` , `ApiPost` or any api debug tool to test the APIs

![ApiPost](./.github/apipost.png)