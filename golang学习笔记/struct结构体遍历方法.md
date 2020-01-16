该repository作为本人读书笔记, 记录知识的获取, 以blog的形式记录下来. 该文库我会不断更新, 如果喜欢的话麻烦点一下`star`.

文章由[https://studygolang.com/articles/5837](https://studygolang.com/articles/5837)启发, 进行展开发散.
利用reflect包对struct结构进行遍历, 对于学习golang有很大的帮助:
完整测试代码如下, 可以用以输出完整的struct成员, 增加递归函数可以很容易的做到多种类型输出:
```go
package main

import (
   "fmt"
   "reflect"
)

type Event struct {
   Topic string
   Data  string
}

type Logs struct {
   Addr   string
   Hash   string
   Height uint64
   Events []Event
}

func main() {
   u := Logs{
      Addr:   "qwrtyuiopasdfghjkl",
      Hash:   "zxcvbnm",
      Height: 1011,
      Events: []Event{{Topic: "123456789", Data: "987654321"}, {Topic: "zz123456789", Data: "zz987654321"}},
   }
   t := reflect.TypeOf(u)
   v := reflect.ValueOf(u)
   for k := 0; k < t.NumField(); k++ {
      fmt.Printf("%s -- %v \n", t.Field(k).Name, v.Field(k).Interface())
      if v.Field(k).Kind() == reflect.Slice {
         t2 := v.Field(k).Type()
         v2 := v.Field(k)
         fmt.Println(t2.String() + " --")
         for i := 0; i < v2.Len(); i++ {
            v3 := v2.Index(i)
            for j := 0; j < v3.NumField(); j++ {
               fmt.Printf("%s -- %v \n", v3.Type().Field(j).Name, v3.Field(j).Interface())
            }
         }
      }
   }
}
```
编译结果:

```shell
Addr -- qwrtyuiopasdfghjkl 
Hash -- zxcvbnm 
Height -- 1011 
Events -- [{123456789 987654321} {zz123456789 zz987654321}] 
[]main.Event --
Topic -- 123456789 
Data -- 987654321 
Topic -- zz123456789 
Data -- zz987654321
```
对于struct嵌套[]struct的处理方式:
```go
if v.Field(k).Kind() == reflect.Slice {
   t2 := v.Field(k).Type()
   v2 := v.Field(k)
   fmt.Println(t2.String() + " --")
   for i := 0; i < v2.Len(); i++ {
      v3 := v2.Index(i)
      for j := 0; j < v3.NumField(); j++ {
         fmt.Printf("%s -- %v \n", v3.Type().Field(j).Name, v3.Field(j).Interface())
      }
   }
}
```
**注意: 不可导出成员(小写)不可用Interface()方法**