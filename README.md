# velocity 相似模板引擎





### 1、#if 、#elseif 、#else   #end结尾
#### #if  支持层级循环嵌套
```vm
#if(1==1)
    1==1
#elseif(1==2)
    1==2
#end
```


### 2、#set
#### #set 支持、字符串、整数、小数、布尔、数组、Map、算数表达式、逻辑表达式
```vm
#set($name = "Alice")
#set($age = 25)
#set($isMember = true)
#set($cart = ["Book", "Pen", "Notebook"])
#set($profile = {"username": "alice123", "email": "alice@example.com"})
#set($discount = $price * 0.1)
#set($isEligible = $age >= 18 && $isMember)
```

### 3、#foreach  #end结尾
#### #foreach 支持层级循环嵌套
##### 模板内容
```vm
array:
#set($array = ["apple", "banana", "cherry"])
#foreach($arrayItem in $array)
    index:${arrayItem.index} count:${arrayItem.count} first:${arrayItem.first} last:${arrayItem.last} hasNext:${arrayItem.hasNext} item:${arrayItem}
#end
```
##### 模板输出
```vm
array:
    index:0 count:1 first:true last:false hasNext:true item:"apple"
    index:1 count:2 first:false last:false hasNext:true item:"banana"
    index:2 count:3 first:false last:true hasNext:false item:"cherry"
```

##### 模板内容
```vm
#foreach($project in $project_list)
${project.count}    project_name:${project.name}
    #foreach($user in $project.user_list)
    ${user.count}    user_name:${user.name}
    #end
#end
```
##### 模板输出
```vm
1    project_name:项目1    
    1    user_name:张三    
    2    user_name:李四    
2    project_name:项目2    
    1    user_name:王五    
    2    user_name:小李子    

```

### 4、注释
```vm
<!-- 这是第一段注释 -->    会渲染到结果中
##这是第一段注释           不会渲染到结果中 
#* 这是第一段注释 *#       不会渲染到结果中
```
