use std::collections::HashMap;

use jni::objects::{JMap, JObject, JString, JValueGen};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_kacz_test_Test_hello<'local>(
    mut env: JNIEnv<'local>,
    object: JObject<'local>,
) -> jstring {
    let name: JValueGen<JObject> = env
        .get_field(object, "name", "Ljava/lang/String;")
        .expect("Couldn't get name field");
    let name = name.l().expect("Couldnt unwrap to object");
    // let name: JString = JString::from(name);
    let name: String = env
        .get_string(&name.into())
        .expect("Couldn't get java string")
        .into();
    let output = env
        .new_string(format!("Hello, {}!", name))
        .expect("Couldn't create java string!");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_kacz_test_Test_differentHellos<'local>(
    mut env: JNIEnv<'local>,
    _called_object: JObject<'local>,
    //input
    names: JObject<'local>,
) {
    let names = env.get_list(&names).expect("Couldn't get JList!");
    let capacity = names.size(&mut env).expect("Couldnt get JLists size!");

    let _ = env.with_local_frame(capacity, |env| -> Result<(), jni::errors::Error> {
        let mut iterator = names.iter(env)?;
        while let Some(obj) = iterator.next(env)? {
            let name: String = env
                .get_string(&obj.into())
                .expect("Couldn't get java string")
                .into();
            println!("Hello {}!", name);
        }
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_kacz_test_Test_frequencies<'local>(
    mut env: JNIEnv<'local>,
    _called_object: JObject<'local>,
    //input
    input_string: JString<'local>,
) {
    let input: String = env
        .get_string(&input_string.into())
        .expect("Couldn't get java string")
        .into();
    let mut rs_map: HashMap<char, usize> = HashMap::new();
    input.chars().for_each(|c| {
        if rs_map.contains_key(&c) {
            rs_map.insert(c, rs_map.get(&c).unwrap() + 1);
        } else {
            rs_map.insert(c, 1);
        }
    });
    // return Ok(());
}


#[no_mangle]
pub extern "system" fn Java_com_kacz_test_Main_rsFrequencies<'local>(
    mut env: JNIEnv<'local>,
    _called_object: JObject<'local>,
    //input
    input_string: JString<'local>,
) {
    let input: String = env
        .get_string(&input_string.into())
        .expect("Couldn't get java string")
        .into();
    let mut rs_map: HashMap<char, usize> = HashMap::new();
    input.chars().for_each(|c| {
        if rs_map.contains_key(&c) {
            rs_map.insert(c, rs_map.get(&c).unwrap() + 1);
        } else {
            rs_map.insert(c, 1);
        }
    });
}
