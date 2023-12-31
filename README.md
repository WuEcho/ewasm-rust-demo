
## 合约开发规范

ewasm 合约接口规范由以太坊定制，指定模块结构等信息，MetaVM 严格遵循此规范，具体如下

### 数据类型

禁止使用浮点数，兼容 evm 中规定的数据类型，例如：

* bytes : 不定长字节数组
* address : 160 bit 数字，在内存中以 20字节 小字节无符号整型表示
* u128 : 128 bit 数字，在内存中以 16字节 小字节无符号整型表示
* u256 : 256 bit 数字，在内存中以 32字节 小字节无符号整型表示

### 格式

每个合约必须存储为 wasm 字节码

### 导入模块

规定合约 `import` 的范围仅限于 [EEI](https://github.com/ewasm/design/blob/master/eth_interface.md) 提供的模块，`ethereum` 名称空间以外的包只允许使用 `debug` ，在生产环境中 `debug` 也应被禁止使用

### 导出函数 

每个合约必须导出两个函数(只能导出两个函数)

* memory : 可供 [EEI](https://github.com/ewasm/design/blob/master/eth_interface.md) 写入的共享内存
* main : 一个入口函数，没有参数也没有返回值，将被 VM 执行

要关闭 `wasm` 的 `start function` 功能，开启它会影响 `ewasm` 在启动前获取合约内存地址指针的功能


### 关于 ABI

>我们看到有关导出函数的规定与 `solidity` 合约中定义的 `ABI` 有些不一样，<br>
>`solidity` 合约根据方法签名来生成相应的 `ABI` 以便对合约中的函数进行调度，<br>
>这在 `ewasm` 看来似乎行不通，因为只有一个 `main` 函数被导出了， <br>
>如何使用 `main` 函数之外的函数呢？我们很自然就想到了使用合约的 `input` 来 <br>
>定义目标方法和输入参数，事实上 `solidity` 也是这么做的，只是我们把这个灵活性 <br>
>交还给开发者实现，以统一的 `main` 函数作为入口，然后自行封装 `input` 序列化方案，<br>
>在后面的例子中我们可以看到更加灵活的方式。<br>



## 开发环境安装

MetaVM 使用 rust 作为 ewasm 合约开发语言，并通过 rust 工具链对合约进行编译，具体安装与使用流程如下

1. 安装 rustup

```
curl https://sh.rustup.rs -sSf | sh
```

>注意，在安装脚本执行时要选择 `nightly` 频道，否则无法完成后续工具安装 
>安装时如果 path 处理失败，需要手动加载一下环境变量 : `source $HOME/.cargo/env`


2. 安装 rust 标准库

```
rustup component add rust-src
```

3. 安装 wasm-pack 编译工具

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

4. 安装 wasm 后期处理工具

```bash
$> git clone https://github.com/WuEcho/wasm-chisel.git
$> cd wasm-chisel
$> cargo build --release
```

编译成功后会在 `target/release` 目录下找到 `chisel` 程序，确保将其复制到 `$PATH` 目录


__有关 Rust 更多内容请参考：__

https://www.rust-lang.org/zh-CN/learn/get-started


## WASM 样例合约 

此步骤之前请确保合约开发环境已经安装完成，我们接下来会用到 `cargo` 创建合约，
使用 `wasm-pack` 来编译合约，使用 `chisel` 对合约进行后期处理

### 创建 hello-wasm 合约

假设工作目录为 `/Users/wuxinyang/Desktop/MyGo/src/rust` ，在终端下进入目录

```bash
$> cargo new --lib hello-wasm
$> cd hello-wasm
$> touch chisel.yml
```

编辑 `chisel.yml` 文件，填入下文中的内容，其中 `file` 属性为 `hello-wasm` 合约编译后产生的二进制文件：

```yml
hello:
  file: "/Users/wuxinyang/Desktop/MyGo/src/rust/hello_wasm_bg.wasm"
  remapimports:
    preset: "ewasm"
  trimexports:
    preset: "ewasm"
  verifyimports:
    preset: "ewasm"
  verifyexports:
    preset: "ewasm"
  repack:
    preset: "ewasm"
``` 

### 添加依赖

一个 `wasm` 合约至少要依赖两个开发包，`ewasm-rust-api` 和 `wasm-bindgen` ，
前者提供 `api` 与 MetaVm 交互，后者负责编译 rust 为 wasm ; 

`MetaVm` 对 `eei` 进行了扩展，需要使用 `MetaVm` 提供的 `ewasm-rust-api`

编辑 `hello-wasm/Cargo.toml` 文件，添加依赖到 `dependencies` 下，并且配置 `profile.release` 以优化编译结果

```toml
[package]
name = "hello-wasm"
version = "0.1.0"
authors = ["WuEcho <emailforecho@163.com>"]
edition = "2022"
publish = false

[dependencies]
wasm-bindgen = "0.2"
ewasm_api = { git = "https://github.com/WuEcho/ewasm-rust-api", tag = "0.9" }

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = 'z'
debug = false
rpath = false
debug-assertions = false
codegen-units = 1
lto = true
```

使用 `cargo check` 检查并下载依赖

```bash
$> cargo check
    Updating crates.io index
   Compiling proc-macro2 v0.4.30
   Compiling unicode-xid v0.1.0
   Compiling syn v0.15.42
   Compiling wasm-bindgen-shared v0.2.48
   Compiling log v0.4.8
   Compiling cfg-if v0.1.9
   Compiling lazy_static v1.3.0
   Compiling libc v0.2.60
   Compiling bumpalo v2.5.0
    Checking void v1.0.2
   Compiling wee_alloc v0.4.4
   Compiling wasm-bindgen v0.2.48
    Checking memory_units v0.4.0
    Checking unreachable v1.0.0
    Checking ewasm_api v0.9.1 
   Compiling quote v0.6.13
   Compiling wasm-bindgen-backend v0.2.48
   Compiling wasm-bindgen-macro-support v0.2.48
   Compiling wasm-bindgen-macro v0.2.48
    Checking hello-wasm v0.1.0 (/Users/wuxinyang/Desktop/MyGo/src/rust/hello-wasm)
    Finished dev [unoptimized + debuginfo] target(s) in 16.90s
```

### 编写合约代码

至此合约的开发工作已经准备完毕，接下来我们将在合约中实现 `put / get` 函数，以及一个简单的计数器
用来演示通过合约存储 `k/v` 值并根据 `k` 获取值，以及如何通过 `contract.input` 来进行不同函数的调度

编辑 `hello-wasm/src/lib.rs` 添加合约代码 

```rust
use wasm_bindgen::prelude::*;
use ewasm_api::types::*;
use ewasm_api::metavm::utils::*;

// 为 counter 定义一个 32位的 key
const COUNTER_KEY: Bytes32 = Bytes32 { bytes: [255; 32] };

// 每次方法被调用时都会执行一个 counter++ 操作，在链上记录执行次数，以测试状态写入操作
// EEI 函数回调 storageLoad / storageStore
fn inc_counter() {
    // storage_load 为 eei 中提供的函数，约束 k/v 均为 32byte 
    // 此处将获取 key 对应的 Bytes32 类型的 value
    let old_v = ewasm_api::storage_load(&COUNTER_KEY);
    // 此方法由 ewasm_api::metavm::utils 名称空间所提供
    // 用来将 32 byte 字节数组转换为对应的整型
    let old_i = bytes_to_uint(&old_v.bytes[..]);
    let new_i = old_i + 1;
    // 此方法由 ewasm_api::metavm::utils 名称空间所提供
    // 用来将 uint32 转换为 32 byte 数组
    let val = u32_to_bytes32(new_i as u32);
    let value = Bytes32 { bytes: val };
    // storage_store 为 eei 中提供的函数，约束 k/v 均为 32byte
    // 用来保存 k/v 到当前合约的状态库
    ewasm_api::storage_store(&COUNTER_KEY, &value);
}


// EEI 函数回调 storageLoad
fn get_counter() {
    let v = ewasm_api::storage_load(&COUNTER_KEY);
    // 如果想将合约的执行结果返回给调用方，不需要使用 return 也无需在方法签名中指明
    // 必须使用 eei 中规定的 finish_data 函数
    ewasm_api::finish_data(&v.bytes[..]);
}

fn put_data() {
    // input 格式为 "put:key,value"
    let input = ewasm_api::calldata_acquire();
    let data = String::from_utf8(input).expect("error_params");
    // 将 input 分割为 ["put","key,value"]
    let sd: Vec<&str> = data.split(":").collect();
    if sd.len() > 1 {
        // 将 "key,value" 分割为 ["key","value"]
        let sp: Vec<&str> = sd[1].split(",").collect();
        if sp.len() > 1 {
            let k = sp[0].trim();
            let v = sp[1].trim();
            // storage_store 为 metavm 提供的扩展函数
            // 用来将不限制大小的 key / value 保存在合约状态中
            // 值得注意的是此方法的 gas 是以数据大小来计算的
            // 每 32byte 数据所使用的 gas 与 storage_store 相同
            ewasm_api::metavm::storage_store(k.as_bytes(), v.as_bytes());
        }
    }
}

fn get_data() {
    // input 格式为 "get:key"
    let input = ewasm_api::calldata_acquire();
    let data = String::from_utf8(input).expect("error_params");
    // 将 input 分割为 ["get","key"]
    let sd: Vec<&str> = data.split(":").collect();
    if sd.len() > 1 {
        let k = sd[1].trim();
        // storage_load 为 metavm 提供的扩展函数
        // 用来获取 key 对应的不限制大小的 value
        // 值得注意的是此方法的 gas 是以数据大小来计算的
        // 每 32byte 数据所使用的 gas 与 storage_store 相同
        let v: Vec<u8> = ewasm_api::metavm::storage_load(k.as_bytes());
        // 将合约执行结果返回给调用端
        ewasm_api::finish_data(&v[..]);
    }
}


//fn constructor() {}

// 同 solidity 中的匿名函数，每次给这个合约转账时都会回调这个函数
// 如果需要使用匿名函数在收到转账时做特殊处理，则可实现这个函数 
fn anonymous() {
    // TODO 不需要返回任何值
    
}

// 合约入口 : 必须使用 #[wasm_bindgen] 注解来声明导出 main 函数
#[wasm_bindgen]
pub fn main() {
    // 当合约通过 tx 调用时表示需要改变状态，此时计数器会加一，否则无效
    inc_counter();
    // 获取本次合约调用的 contract.input
    let input = ewasm_api::calldata_acquire();
    // 当 create 合约时 input 始终为空
    // 当给合约发送普通转账交易时，input 也应为空
    if !input.is_empty() {
        // 本 demo 使用了文本协议来序列化 input
        // 格式为: "目标函数:参数1,参数2,参数n"
        // 解析
        let data = match String::from_utf8(input) {
            Ok(s) => s,
            Err(e) => e.to_string(),// 也可以在此处终止合约
        };
        // 将 input 分割为 ["目标函数","参数1,参数2,参数n"]
        let sd: Vec<&str> = data.split(":").collect();
        // 通过这个匹配可以看出我们这个合约对外暴漏 3 个函数，函数名称不区分大小写：
        //      GETCOUNTER : 通过 eth_call 调用，用来获取计数器结果
        //      PUT : 通过 tx 向合约中添加一个 k/v 对，具体参数格式为 "put:k,v"
        //      GET : 通过 eth_call 调用，获取 k 对应的 v，具体参数格式为 "get:k"
        // 当方法名得不到匹配时，会返回 "METHOD_NOT_FOUND" 标识
        match sd[0].trim().to_uppercase().as_str() {
            "GETCOUNTER" => get_counter(),
            "PUT" => put_data(),
            "GET" => get_data(),
            _ => ewasm_api::finish_data(String::from("METHOD_NOT_FOUND").as_bytes()),
        }
    } else {
        // 当 input 为空时，调度匿名函数
        anonymous();
    }
}
```


### 编译合约

在控制台进入 `hello-wasm` 工程目录，编译并完成后期处理

```bash
$> wasm-pack build
$> chisel run
```

以上步骤将在 `/Users/wuxinyang/Desktop/MyGo/src/rust` 目录中得到 `hello_wasm_bg.wasm` 文件，接下来我们去链上部署这份合约

### 哨兵合约示例

 rust代码 https://github.com/ewasm/sentinel-rs
 solodity代码 https://github.com/ewasm/sentinel-governance

### 部署以及调用

以构造交易体并发送交易给链上的方式进行部署，构造交易的代码如下：

```go
import (
"github.com/ethereum/go-ethereum/ethclient"
"github.com/ethereum/go-ethereum/core/types"
"github.com/ethereum/go-ethereum/common"
"testing"
"io/ioutil"
"encoding/hex"
)
func TestDeploy(t *testing.T) {
	fp := "/Users/wuxinyang/Desktop/MyGo/src/rust/hello-wasm/pkg/hello_wasm_bg.wasm"
	ctx := context.Background()
	code, err := ioutil.ReadFile(fp)
	if err != nil {
		t.Error(err)
		return
	}
	t.Log(code)
	t.Log(hex.EncodeToString(code))
	client, err := ethclient.Dial(ipc)
	if err != nil {
		t.Error(err)
		return
	}
	nonce, err := client.PendingNonceAt(ctx, addr)
	if err != nil {
		t.Error(err)
		return
	}
	tx, _ := types.SignTx(
		types.NewContractCreation(
			nonce,
			new(big.Int),
			11826015,                                          //gasLimit
			new(big.Int).Mul(big.NewInt(1e9), big.NewInt(18)), // gasPrice
			code),
		signer,
		prvKey)
	t.Log(nonce, tx)
	err = client.SendTransaction(ctx, tx)
	t.Log(err, len(code), tx.Hash().Hex())
}

//调用方法
func TestCall(t *testing.T) {
	to := common.HexToAddress("0xda3df11d916ffba3a1289cef66a7f142ec5d0f76")
	ctx := context.Background()
	client, err := ethclient.Dial(ipc)
	if err != nil {
		t.Error(err)
		return
	}
	nonce, err := client.PendingNonceAt(ctx, addr)
	if err != nil {
		t.Error(err)
		return
	}
	_tx := types.NewTransaction(
		nonce,
		to,
		big.NewInt(1),
		params.GenesisGasLimit,                            // gasLimit
		new(big.Int).Mul(big.NewInt(1e9), big.NewInt(18)), // gasPrice
		[]byte("put:helloworld,wuecho@163.com"))
	//get:helloworld
	tx, _ := types.SignTx(_tx, signer, prvKey)
	t.Log(nonce, tx)
	err = client.SendTransaction(ctx, tx)
	t.Log(err, tx.Hash().Hex())

}
```



