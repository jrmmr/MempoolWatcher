# Mempool Watcher 

Create a MempoolWatcher object by passing the multicast address to listen.
MempoolWatcher emits `MempoolEvent::Trx` event at each new transaction with the transaction datas.
MempoolWatcher parse each transaction to extract `Message` and emits `MempoolWatcher::NewMsg` for each message.

### Example

``` rust
let node = MempoolWatcher::new("225.0.0.1:1234");

    if let Ok(receiver) = node.start() {
        loop {
            if let Some(event) = receiver.recv().await {
                match event {
                    // get Trx, Trx may contains several messages
                    MempoolEvent::NewTrx(_trx, _raw_trx) => {}
                    // get message
                    MempoolEvent::NewMsg(msg) => {
                        if let Some(type_) = msg["@type"].as_str() {
                            if type_ == "/terra.wasm.v1beta1.MsgExecuteContract" {
                                let sender = msg["sender"].as_str().unwrap_or("");
                                let contract = msg["contract"].as_str().unwrap_or("");
                                let execute_msg = msg["execute_msg"].as_object().unwrap();

                                println!("==================================================");
                                println!("sender      : {}", sender);
                                println!("contract    : {}", contract);
                                println!("execute_msg : {:?}", execute_msg);
                                println!("==================================================");
                            }
                        }
                    }
                }
            }
        }
    }
```

### TODO

- [ ] Add `stop` function



inspired by [twelvepool](https://github.com/setten-io/twelvepool)
