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



### 使用示例如下
```rust
pub fn test1() {
    let result = render_default_path("tests/comment/comment.vm");
    if let Ok(content) = result {
        println!("----------------------------------------------------------------\n{}", content);
        println!("----------------------------------------------------------------")
    }
}

pub fn test2() {

    let template_path = "tests/comment/comment.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };
    let result = render_default(template.as_str());
    if let Ok(content) = result {
        println!("----------------------------------------------------------------\n{}", content);
        println!("----------------------------------------------------------------")
    }

}

#[derive(Debug,Serialize,Deserialize)]
struct Project{
    name:String,
    user_list:Vec<ProjectUser>,
}
#[derive(Debug,Serialize,Deserialize)]
struct ProjectUser{
    name: String,
}
#[derive(Debug,Serialize,Deserialize)]
struct Template{
    project_list:Vec<Project>,
}

#[test]
fn test4(){
    
    let user1 = ProjectUser{
        name: "张三".to_string(),
    };
    let user2 = ProjectUser{
        name: "李四".to_string(),
    };

    let user3 = ProjectUser{
        name: "王五".to_string(),
    };
    let user4 = ProjectUser{
        name: "小李子".to_string(),
    };


    let p1 = Project{
        name: "项目1".to_string(),
        user_list: vec![user1,user2],
    };

    let p2 = Project{
        name: "项目2".to_string(),
        user_list: vec![user3,user4],
    };

    let entity = Template{
        project_list: vec![p1,p2],
    };

    for i in 0..1000{
        let output = render_from_path("tests/foreach/foreach_array.vm",&entity);
        if let Ok(output) = output {
            println!("------------------------------------------------------------------\n{}", output);
            println!("------------------------------------------------------------------");
        }
    }
    
}
```