## 第八课作业

**(7 分)** 

利用 off-chain worker 的概念，算出 1^2 + 2^2 + 3^2 + 4^2 + .... 

* 第一個區塊導入結束時算出：1^2
* 第二個區塊導入結束時算出：1^2 + 2^2
* 第三個區塊導入結束時算出：1^2 + 2^2 + 3^2

代码片段:
 ![](./lsn8_10pallet.png)
输出:
 ![](./lsn8_11out.png)
UI显示:
 ![](./lsn8_12ui.png)

當第三個區塊導入結束時，能對鏈上發出請求：

* sum(0) = 1
* sum(1) = 5
* sum(2) = 14

计算要在链下完成，链上只用作储存。提交到鏈上時用具簽名交易。

**(3 分)** 

附加题：写两个单元测试：

* 第一个是测试链下的计算逻辑
* 第二个是测试链上的函数

代码:
 ![](./lsn8_20test_code.png)
UI显示:
 ![](./lsn8_21test_out.png)
