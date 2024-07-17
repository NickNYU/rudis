<a name="iSXbN"></a>
## Context
Redis的EventLoop包含两类Event：

1. FileEvent（其实说适合翻译为 IO Event）
2. TimeEvent

所有的Event都在EventLoop中处理，整体的概览如下<br />
![ae-main.png](./picture/ae-main.png "ae-main.png")

## Goal

1. 将EventLoop和AeMain抽象成模块
2. 用tokio-mio模拟Redis实现不同平台的aeApiPoll

## 改动