# checknote870_new
- 参照下面网址进行学习
- https://docs.github.com/zh/get-started/quickstart/hello-world
- 在创建repository时，选public时，git clone 是可以直接下载的
- 选private时，git clone 需要账号和密码

-     

# 规范
- 写流程图
- 按模块，定义模块功能，模块名称，模块输入参数，返回值。
- 尽量把每个模块用一个函数，在一个模块的一个函数内解决，不要分成小时粒度，分钟力度


- filename to table 输入文件名，获得解压后的数据表，给写入数据模块
# 1.  main 模块
        获取时间，并生成时间，调用输出；
        在main.rs中做创建数据库，没有数据库，要创建默认数据库，有数据库，则打开数据库测试读取表中字段，数据库名在命令行中指定，数据库中有几个表，
        create database rust_count
        create table filecount("包表时间"， 时间戳，条数）
        create table
# 2.  时间列表模块
        函数名（参数，返回值） timelist(String)  -> Vector
                          String 包含 年月日 小时 分钟
# 3. 文件列表过滤模块
        函数名（参数，返回值） allfilelist_to_daylist(Vector)  -> Vector
# 4. 处理每行，获取需要解压的文件名
                      lineparse(Vector) -> Vector
# 5. 下载文件模块，解压模块，计数模块
            下载文件模块  写个ftp，解压模块 写 zip，计数模块 用read_line
# 6. 写入数据库模块
             数据库用sqlite
             数据库在main.rs测试 成功后，在写入数据库模块中，再次测试能否成功打开和读取表中字段，然后再进行写入数据库

# 7.读取数据库模块
             读取，包表时间与时间戳组成hashmap，与最新的文件列表进行比较，以决定到底是更新，还是不更新
# 8.  输出，从数据库读取，然后再展示
             判断是否是缺数据在输出这步做，根据过往数据来判断今天，哪个时间点的数据条数是多还是少
             生成什么样的格式待定
##      这个是不是要定时执行呢？每个小时执行一次来解决数据什么时候入库问题
##      如果不定时更新，那就要挑某个时间，根据数据库中已有的文件的最新时间跟文件列表中最新时间进行比对后更新。




