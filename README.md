# OpenArk
Proof-of-concept fully open arknights themed game  
随便整点，感兴趣就加入,不要太当真  
Just randomly poking around, feel free to mess these things around with me.   
You'd better not take this seriously or you'll get rather disappointed.

## 项目状态  
暂停：等待依赖更新至bevy 0.7  

## 构建项目
开发模式构建：
``cargo build --features bevy/dynamic ``  
打上``--feature bevy/dynamic``可将数分钟的编译用时缩短到数秒。  
发布模式构建：
``cargo build --release``  
将``build``换成``run``即可直接运行  
如果没有安装Rust工具链请前往[Rust官网](https://www.rust-lang.org)获取安装指南。  
如果想要更激进的优化敬请阅读[Bevy Book](https://bevyengine.org/learn/book/getting-started/setup/)。  

## 想要加入  
  来，都可以来，只要你有好的态度，啥都可以干。  
  急需建模，[Blender](https://www.blender.org/)或者[BlockBench](https://github.com/JannisX11/blockbench)。  
  要求就BlockBench那个README里的例子就成（体素），不需要几万个面，也不需要2K材质。  
  ~~由于引擎限制不支持骨骼动画所以没必要在这上面花时间。~~   
  bevy 0.7目前支持骨骼动画     
  模型需要能以``.gltf``或``.glb``导出，如果是``.obj``那可能需要更多工作。   

## FAQ
  - 为什么不使用Unity/Unreal？  
  这两个引擎都是商业引擎，虽然本项目不盈利不需要支付使用费用，但两个引擎的开放程度比不上Bevy。  
  我清楚这两个引擎都有非常非常非常非常丰富的教程和资源，但是我头比较铁。
  - 为什么不使用Godot？  
  cpp是把双刃剑。
  - 什么时候我能玩到正式版？  
  下辈子。
  - 有交流群吗？  
  没有，IM效率低下。有什么事开Issue就行，或者发邮件。  
  - 哪里赞助？  
  [这里](https://www.bilibili.com/video/BV1GJ411x7h7)
