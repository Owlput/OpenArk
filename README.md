# OpenArk
Proof-of-concept fully open arknights themed game  
随便整点，感兴趣就加入,不要太当真  
Just randomly poking around, feel free to mess these things around with me.   
You'd better not take this seriously or you'll get rather disappointed.

## 构建项目
开发模式构建：
``cargo build --features bevy/dynamic ``  
打上``--feature bevy/dynamic``可将数分钟的编译用时缩短到数秒。  
发布模式构建：
``cargo build --release``  
将``build``换成``run``即可直接运行  
如果没有安装Rust工具链请前往[Rust官网](https://www.rust-lang.org)获取安装指南。  
如果想要更激进的优化敬请阅读[Bevy Book](https://bevyengine.org/learn/book/getting-started/setup/)。  

## FAQ
  - 为什么不使用Unity/Unreal？  
  这两个引擎都是商业引擎，虽然本项目不盈利不需要支付使用费用，但两个引擎的开放程度比不上Bevy。  
  我清楚这两个引擎都有非常非常非常非常丰富的教程和资源，但是我头比较铁。
  - 为什么不使用Godot？
  cpp是把双刃剑。
  - 什么时候我能玩到正式版？
  下辈子。
