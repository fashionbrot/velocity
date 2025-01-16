use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use velocity_template;
use velocity_template::{read_file, render, render_from_path};
use crate::log_config;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String
}

#[test]
pub fn test(){
    let template_path = "tests/if_test/if.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}",template);

    let user1 = User {
        username: "张三".to_string(),
    };
    let user2 = User {
        username: "李四".to_string(),
    };
    let user_list = vec![user1,user2];

    // 将 Vec<User> 转换为 serde_json::Value::Array
    let user_list_json = serde_json::to_value(user_list).expect("Failed to serialize users");

    let mut engine = HashMap::<String, Value>::new();
    engine.insert(String::from("age"), Value::Number(Number::from(17)));
    engine.insert(String::from("userList"), user_list_json);
    engine.insert(String::from("one"), Value::Number(Number::from(1)));
    engine.insert(String::from("rust"), Value::String("rust 2025".to_string()));
    engine.insert(String::from("swagger2Enable"), Value::Bool(true));


    let output = render(&template,&mut engine);
    if let Ok(output) = output {
        println!("output:\n{}",output);
    }

}


#[test]
fn test2(){
    log_config::print_debug_log();
    let json_data = r#"
    {
        "sourceSetJava": ".src.main.java",
        "lombokEnable": true,
        "mybatisPlusEnable": true,
        "serviceOut": ".service",
        "mapperXmlEnable": true,
        "tableLogicDeleteValue": 1,
        "fieldInsertFillNames": "createDate",
        "pageHelperEnable": true,
        "mapperOut": ".mapper",
        "mapperXmlAliasEnable": true,
        "requestEnable": true,
        "tableNameDescription": "测试表",
        "responseClassName": "Response",
        "entityEnable": true,
        "allColumn": "id,`name`,address,price,score,brand,city,star_name,business,latitude,longitude,pic",
        "mapperSuffix": "Mapper",
        "responseSuffix": "Response",
        "versionFieldName": "version",
        "entityOut": ".entity",
        "responseOut": ".response",
        "sourceSetResources": ".src.main.resources",
        "mapperEnable": true,
        "excludePrefix": "",
        "tableName": "tb_hotel",
        "compileType": "gradle",
        "swagger2Enable": false,
        "controllerSuffix": "Controller",
        "out": "d:\\\\test",
        "dateTimeFormatValue": "yyyy-MM-dd HH:mm:ss",
        "entitySuffix": "Entity",
        "requestOut": ".request",
        "primaryKeyType": "Long",
        "serviceSuffix": "Service",
        "serviceImplEnable": true,
        "mapperXmlSelectByIdEnable": false,
        "mapperXmlInsertEnable": false,
        "basicEnable": false,
        "permissionOut": ".annotation",
        "mapperXmlUpdateByIdEnable": false,
        "camelCaseTableName": "tbHotel",
        "requestSuffix": "Request",
        "className": "TbHotel",
        "dateFormatValue": "yyyy-MM-dd",
        "serviceEnable": true,
        "serviceImplOut": ".service.impl",
        "permissionEnable": true,
        "allColumnAlias": "t.id,t.`name`,t.address,t.price,t.score,t.brand,t.city,t.star_name,t.business,t.la",
        "mapperXmlOut": ".mapper.xml",
        "swagger3Enable": true,
        "timeFormatValue": "HH:mm:ss",
        "customInsertInterfaceEnable": true,
        "customDeleteByIdInterfaceEnable": true,
        "permissionClassName": "MarsPermission",
        "customSelectByIdInterfaceEnable": true,
        "deleteFieldName": "delFlag",
        "serialVersionUIDEnable": true,
        "mapperXmlInsertsEnable": false,
        "customListInterfaceEnable": true,
        "tableLogicValue": 0,
        "packageOut": "com",
        "tableFieldList": [
            {
                "is_primary": true,
                "column_name": "id",
                "column_name_keyword": "id",
                "hump_column_name": "id",
                "column_type": "bigint",
                "java_column_type": "Long",
                "column_comment": "酒店id"
            },
            {
                "is_primary": false,
                "column_name": "name",
                "column_name_keyword": "`name`",
                "hump_column_name": "name",
                "column_type": "varchar",
                "java_column_type": "String",
                "column_comment": "酒店名称"
            },
            {
                "is_primary": false,
                "column_name": "address",
                "column_name_keyword": "address",
                "hump_column_name": "address",
                "column_type": "varchar",
                "java_column_type": "String",
                "column_comment": "酒店地址"
            },
            {
                "is_primary": false,
                "column_name": "price",
                "column_name_keyword": "price",
                "hump_column_name": "price",
                "column_type": "int",
                "java_column_type": "Integer",
                "column_comment": "酒店价格"
            },
            {
                "is_primary": false,
                "column_name": "score",
                "column_name_keyword": "score",
                "hump_column_name": "score",
                "column_type": "int",
                "java_column_type": "Integer",
                "column_comment": "酒店评分"
            },
            {
                "is_primary": false,
                "column_name": "brand",
                "column_name_keyword": "brand",
                "hump_column_name": "brand",
                "column_type": "varchar",
                "java_column_type": "String",
                "column_comment": "酒店品牌"
            }
        ],
        "customUpdateByIdInterfaceEnable": true,
        "controllerOut": ".controller",
        "mapperXmlSuffix": "Mapper",
        "customPageListInterfaceEnable": true,
        "responseEnable": true,
        "mapperXmlDeleteByIdEnable": false,
        "fieldUpdateFillNames": "updateDate",
        "serialVersionUID": 7937875882129814205,
        "serviceImplSuffix": "ServiceImpl",
        "customDeleteByIdsInterfaceEnable": true,
        "controllerEnable": true,
        "author": ""
    }
    "#;

    // 解析 JSON 数据为 serde_json::Value
    let value: Value = serde_json::from_str(json_data).unwrap();

    // 将 Value 转换为 HashMap<String, Value>
    let mut engine: HashMap<String, Value> = serde_json::from_value(value).unwrap();

    let output = render_from_path("tests/if_test/entity.vm",&mut engine);
    if let Ok(output) = output {
        println!("output:----------------------------------------------------------------\n{}",output);
    }
}

