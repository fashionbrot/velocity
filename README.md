# velocity 模板引擎

### 1、#if 、#elseif 、#else   #end结尾
### 2、#foreach  #end结尾
### 3、#set

#### #if  支持层级循环嵌套
```vm
#if(1==1)
    1==1
#elseif(1==2)
    1==2
#end
```


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

#### #foreach 支持层级循环嵌套
```vm
#set($fruits = ["apple", "banana", "cherry"])

#foreach($fruit in $fruits)
    #set($index = ${foreach.index})
    Index: $index, Fruit: $fruit
#end
```

#### 注释
```vm
<!-- 这是第一段注释 -->    会渲染到结果中
##这是第一段注释           不会渲染到结果中 
#* 这是第一段注释 *#       不会渲染到结果中
```

```rust


```