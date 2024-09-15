use crate::error::MocksError;
use crate::storage::reader::Reader;
use crate::storage::writer::Writer;
use serde_json::{Map, Value};

mod reader;
mod writer;

/// Storage module
#[derive(Default)]
pub struct Storage {
    pub file: String,
    pub overwrite: bool,
    pub value: Value,
}

impl Storage {
    pub fn new(path: &str, overwrite: bool) -> Result<Storage, MocksError> {
        let value = Reader::new(path).read()?;
        Ok(Storage {
            file: path.to_string(),
            overwrite,
            value,
        })
    }

    /// GET method for list
    pub fn get_resource(&self, resource_key: &str) -> Option<Value> {
        self.value.get(resource_key).cloned()
    }

    /// GET method for item
    pub fn get_one(&self, resource_key: &str, item_key: &str) -> Option<Value> {
        match self.value.get(resource_key) {
            None => None,
            Some(value) => {
                let item = if let Value::Array(values) = value {
                    let mut result = None;
                    for v in values {
                        if let Value::Object(target) = v {
                            for (key, value) in target {
                                if key == "id" {
                                    match value {
                                        Value::Number(id_number) => {
                                            if id_number.to_string() == *item_key {
                                                result = Some(v);
                                            }
                                        }
                                        Value::String(id_string) => {
                                            if id_string == item_key {
                                                result = Some(v);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    result
                } else {
                    None
                };

                item.cloned()
            }
        }
    }

    /// POST method
    pub fn upsert(&mut self, resource_key: &str, input: &Value) -> Result<Value, MocksError> {
        match self.value.get_mut(resource_key) {
            None => Err(MocksError::ResourceNotFound()),
            Some(target) => match target {
                Value::Array(ref mut values) => {
                    values.push(input.clone());
                    self.write()?;

                    Ok(input.clone())
                }
                Value::Object(_map) => {
                    let mut new_value = Map::new();
                    match self.value.as_object() {
                        None => Err(MocksError::ExceptionError()),
                        Some(obj) => {
                            for (k, v) in obj {
                                if k == resource_key {
                                    new_value.insert(k.clone(), input.clone());
                                } else {
                                    new_value.insert(k.clone(), v.clone());
                                }
                            }

                            self.value = Value::Object(new_value.clone());
                            self.write()?;

                            Ok(input.clone())
                        }
                    }
                }
                _ => Err(MocksError::ExceptionError()),
            },
        }
    }

    /// PUT method
    pub fn replace(
        &mut self,
        resource_key: &str,
        item_key: &str,
        input: &Value,
    ) -> Result<Value, MocksError> {
        match self.value.get_mut(resource_key) {
            None => Err(MocksError::ResourceNotFound()),
            Some(target) => match target {
                Value::Array(ref mut values) => {
                    let mut new_values: Vec<Value> = Vec::new();
                    for v in values {
                        match v.as_object_mut() {
                            None => {}
                            Some(obj) => {
                                if let Some(id_value) = obj.get("id") {
                                    match id_value {
                                        Value::Number(id_number) => {
                                            if id_number.to_string() == *item_key {
                                                let mut obj = Map::new();
                                                obj.insert(
                                                    "id".to_string(),
                                                    Value::String(item_key.to_string()),
                                                );
                                                if let Value::Object(m) = input {
                                                    for (k, v) in m {
                                                        obj.insert(k.clone(), v.clone());
                                                    }
                                                }

                                                new_values.push(Value::Object(obj));
                                            } else {
                                                new_values.push(Value::Object(obj.clone()));
                                            }
                                        }
                                        Value::String(id_string) => {
                                            if id_string == item_key {
                                                let mut obj = Map::new();
                                                obj.insert(
                                                    "id".to_string(),
                                                    Value::String(item_key.to_string()),
                                                );
                                                if let Value::Object(m) = input {
                                                    for (k, v) in m {
                                                        obj.insert(k.clone(), v.clone());
                                                    }
                                                }

                                                new_values.push(Value::Object(obj));
                                            } else {
                                                new_values.push(Value::Object(obj.clone()));
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }

                    let mut temp = Map::new();
                    temp.insert(resource_key.to_string(), Value::Array(new_values.clone()));
                    match self.value.as_object() {
                        None => {}
                        Some(obj) => {
                            for (k, v) in obj {
                                if k != resource_key {
                                    temp.insert(k.clone(), v.clone());
                                }
                            }
                        }
                    }

                    self.value = Value::Object(temp);
                    self.write()?;

                    match self.get_one(resource_key, item_key) {
                        None => Err(MocksError::ExceptionError()),
                        Some(v) => Ok(v),
                    }
                }
                Value::Object(_map) => {
                    let mut new_value = Map::new();
                    match self.value.as_object() {
                        None => Err(MocksError::ExceptionError()),
                        Some(obj) => {
                            for (k, v) in obj {
                                if k == resource_key {
                                    new_value.insert(k.clone(), input.clone());
                                } else {
                                    new_value.insert(k.clone(), v.clone());
                                }
                            }

                            self.value = Value::Object(new_value.clone());
                            self.write()?;

                            Ok(input.clone())
                        }
                    }
                }
                _ => Err(MocksError::ExceptionError()),
            },
        }
    }

    /// PATCH method
    pub fn update(
        &mut self,
        resource_key: &str,
        item_key: &str,
        input: &Value,
    ) -> Result<Value, MocksError> {
        match self.value.get_mut(resource_key) {
            None => Err(MocksError::ResourceNotFound()),
            Some(target) => {
                match target {
                    Value::Array(ref mut values) => {
                        let mut new_values: Vec<Value> = Vec::new();
                        for v in values {
                            match v.as_object_mut() {
                                None => {}
                                Some(obj) => {
                                    let input_keys = input.as_object().unwrap().keys();
                                    for ik in input_keys {
                                        if !obj.contains_key(ik) {
                                            return Err(MocksError::ObjectNotFound());
                                        }
                                    }

                                    if let Some(id_value) = obj.get("id") {
                                        match id_value {
                                            Value::Number(id_number) => {
                                                if id_number.to_string() == *item_key {
                                                    if let Value::Object(m) = input {
                                                        for (k, v) in m {
                                                            obj.insert(k.clone(), v.clone());
                                                        }
                                                    }

                                                    new_values.push(Value::Object(obj.clone()));
                                                } else {
                                                    new_values.push(Value::Object(obj.clone()));
                                                }
                                            }
                                            Value::String(id_string) => {
                                                if id_string == item_key {
                                                    if let Value::Object(m) = input {
                                                        for (k, v) in m {
                                                            obj.insert(k.clone(), v.clone());
                                                        }
                                                    }

                                                    new_values.push(Value::Object(obj.clone()));
                                                } else {
                                                    new_values.push(Value::Object(obj.clone()));
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }

                        let mut temp = Map::new();
                        temp.insert(resource_key.to_string(), Value::Array(new_values.clone()));
                        match self.value.as_object() {
                            None => {}
                            Some(obj) => {
                                for (k, v) in obj {
                                    if k != resource_key {
                                        temp.insert(k.clone(), v.clone());
                                    }
                                }
                            }
                        }

                        self.value = Value::Object(temp);
                        self.write()?;

                        // get
                        match self.get_one(resource_key, item_key) {
                            None => Err(MocksError::ObjectNotFound()),
                            Some(v) => Ok(v),
                        }
                    }
                    Value::Object(map) => {
                        let input_keys = input.as_object().unwrap().keys();
                        for ik in input_keys {
                            if !map.contains_key(ik) {
                                return Err(MocksError::ObjectNotFound());
                            }
                        }

                        let mut new_value = Map::new();
                        match self.value.as_object_mut() {
                            None => Err(MocksError::ExceptionError()),
                            Some(obj) => {
                                for (k, v) in obj {
                                    if k == resource_key {
                                        let mut temp = Map::new();

                                        if let Value::Object(m) = v {
                                            for (k, v) in m {
                                                temp.insert(k.clone(), v.clone());
                                            }
                                        }

                                        if let Value::Object(m) = input {
                                            for (k, v) in m {
                                                temp.insert(k.clone(), v.clone());
                                            }
                                        }

                                        new_value.insert(k.clone(), Value::Object(temp.clone()));
                                    } else {
                                        new_value.insert(k.clone(), v.clone());
                                    }
                                }

                                self.value = Value::Object(new_value.clone());
                                self.write()?;

                                match self.get_resource(resource_key) {
                                    None => Err(MocksError::ObjectNotFound()),
                                    Some(v) => Ok(v),
                                }
                            }
                        }
                    }
                    _ => Err(MocksError::ExceptionError()),
                }
            }
        }
    }

    pub fn delete(&mut self, resource_key: &str, item_key: &str) -> Result<Value, MocksError> {
        match self.value.as_object_mut() {
            None => Err(MocksError::ExceptionError()),
            Some(value) => {
                let mut temp_map = Map::new();
                let mut deleted_item: Option<Value> = None;
                for (k, v) in value {
                    if k == resource_key {
                        if let Value::Array(items) = v {
                            let mut deleted_values: Vec<Value> = Vec::new();
                            for item in items {
                                if let Value::Object(item_map) = item.clone() {
                                    for (k, v) in item_map {
                                        if k == "id" {
                                            match v {
                                                Value::Number(id_number) => {
                                                    if id_number.to_string() != *item_key {
                                                        deleted_values.push(item.clone());
                                                    } else {
                                                        deleted_item = Some(item.clone());
                                                    }
                                                }
                                                Value::String(id_string) => {
                                                    if id_string != item_key {
                                                        deleted_values.push(item.clone());
                                                    } else {
                                                        deleted_item = Some(item.clone());
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }

                            // No object available for deletion.
                            if deleted_item.is_none() {
                                return Err(MocksError::ObjectNotFound());
                            }

                            temp_map.insert(
                                resource_key.to_string(),
                                Value::Array(deleted_values.clone()),
                            );
                        }
                    } else {
                        temp_map.insert(k.clone(), v.clone());
                    }
                }

                self.value = Value::Object(temp_map.clone());
                self.write()?;

                match deleted_item {
                    None => Err(MocksError::ObjectNotFound()),
                    Some(item) => Ok(item),
                }
            }
        }
    }

    fn write(&mut self) -> Result<(), MocksError> {
        if self.overwrite {
            let writer = Writer::new(&self.file);
            writer.write(&self.value)?;
        }
        Ok(())
    }
}
