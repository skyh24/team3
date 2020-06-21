## 第三课作业  PoE 2

课程里会给出参考资料，大家一定要自己敲一遍**代码**！

注：

1. 提交源代码，运行`cargo test`的测试结果截图，前端UI的截图；
2. 测试应覆盖所有的业务逻辑，如不同的错误场景，以及一切正常的情况下，检查存储的数据是不是预期的那样。
3. 附加题不是必答的，但可以酌情加分。
4. 代码修改在本目录 substrate-node-template 和 substrate-front-end-template 的程序文件里。

第一题：编写存证模块的单元测试代码，包括：

* 创建存证的测试用例；
![](./lsn3_11claim_create.png)
* 撤销存证的测试用例；
![](./lsn3_12claim_revoke.png)
* 转移存证的测试用例；
![](./lsn3_13claim_transfer.png)

全部测试通过
![](./lsn3_0cargo_test.png)

第二题：编写存证模块的UI，包括

* 创建存证的UI
![](./lsn3_21ui_create.png)
* 删除存证的UI
![](./lsn3_22ui_revoke.png)
* 转移存证的UI
![](./lsn3_23ui_transfer.png)

第三题（附加题）：实现购买存证的功能代码：

* 用户A为自己的某个存证记录设置价格；
代码
![](./lsn3_31set_price.png)
界面操作
![](./lsn3_32ui_price.png)

* 用户B可以以一定的价格购买某个存证，当出价高于用户A设置的价格时，则以用户A设定的价格将费用从用户B转移到用户A，再将该存证进行转移。如果出价低于用户A的价格时，则不进行转移，返回错误。
代码
![](./lsn3_33buy_claim.png)
界面,如果不够会Fail
![](./lsn3_34not_enough.png)
购买
![](./lsn3_35ui_buy.png)
