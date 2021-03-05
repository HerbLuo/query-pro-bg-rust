表级权限
行级权限
列级权限

一：
session(or token) -> uid
1. http + 配置
2. code形式
3. nacos

二：
1. 权限控制仅在客户端调用时生效
2. 一个表，如果不定义权限，则表明该表是公共读的。(写入不允许)
3. 控制一个表的读取权限只需 配置表的uid或uid_read字段。  
   该字段可以是虚拟的。  
   例如：  
   咖啡表中没有uid字段，但存在订单order_id字段，订单表中存在user_id代表用户id。  
   ```json
   {
     "table": "coffee",
     "uid_read": "order.user_id",
     "joiners": "left join order on coffee.order_id = order.id"
   }
   ```
4. 控制某一列的读取权限只需增加column字段
   ```json
   {
     "table": "coffee",
     "column": "name"
   }
   ```
5. 写入类似，但先不做，因为一般不允许前端端直接写入数据库表
三：用户组
1. 数据权限还可以分配给某个组
   ```json
   {
     "table": "coffee",
   
     "joiners": "left join department on department.",
     "groups": ["管理员", ["总公司"], ["上海分公司","财务部"], ["上海分公司","业务部"], ["上海市政府大楼店"]]
   }
   ```
   


`
public_write
`

所属用户或所属组

`uid `

对于每一条数据，

数据类型有：

    私有数据
    用户可访问数据
    公共读数据
    公共写数据
    公共读写数据

所有者权限
组权限
其它人权限
