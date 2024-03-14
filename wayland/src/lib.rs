// Copyright 2017 Avraham Weinstock
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    borrow::Cow,
    error::Error,
    ffi::c_void,
    sync::{Arc, Mutex},
};

pub use smithay_clipboard::mime::{AllowedMimeTypes, AsMimeTypes, MimeType};

pub struct Clipboard {
    context: Arc<Mutex<smithay_clipboard::Clipboard>>,
}

impl Clipboard {
    pub unsafe fn connect(display: *mut c_void) -> Clipboard {
        let context = Arc::new(Mutex::new(smithay_clipboard::Clipboard::new(
            display as *mut _,
        )));

        Clipboard { context }
    }

    pub fn read(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.context.lock().unwrap().load_text()?)
    }

    pub fn read_primary(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.context.lock().unwrap().load_primary_text()?)
    }

    pub fn write(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        self.context.lock().unwrap().store_text(data);

        Ok(())
    }

    pub fn write_primary(
        &mut self,
        data: String,
    ) -> Result<(), Box<dyn Error>> {
        self.context.lock().unwrap().store_primary_text(data);

        Ok(())
    }

    pub fn write_data<T: AsMimeTypes + Send + Sync + 'static>(
        &mut self,
        data: T,
    ) -> Result<(), Box<dyn Error>> {
        self.context.lock().unwrap().store(data);

        Ok(())
    }

    pub fn write_primary_data<T: AsMimeTypes + Send + Sync + 'static>(
        &mut self,
        data: T,
    ) -> Result<(), Box<dyn Error>> {
        self.context.lock().unwrap().store_primary(data);

        Ok(())
    }

    pub fn read_data<T: AllowedMimeTypes + 'static>(
        &self,
    ) -> Result<T, Box<dyn Error>> {
        Ok(self.context.lock().unwrap().load()?)
    }

    pub fn read_primary_data<T: AllowedMimeTypes + 'static>(
        &self,
    ) -> Result<T, Box<dyn Error>> {
        Ok(self.context.lock().unwrap().load_primary()?)
    }

    pub fn read_primary_raw(
        &self,
        allowed: Vec<String>,
    ) -> Result<(Vec<u8>, String), Box<dyn Error>> {
        Ok(self
            .context
            .lock()
            .unwrap()
            .load_primary_raw(
                allowed
                    .into_iter()
                    .map(|s| MimeType::from(Cow::Owned(s)))
                    .collect::<Vec<_>>(),
            )
            .map(|(d, m)| (d, m.to_string()))?)
    }

    pub fn read_raw(
        &self,
        allowed: Vec<String>,
    ) -> Result<(Vec<u8>, String), Box<dyn Error>> {
        Ok(self
            .context
            .lock()
            .unwrap()
            .load_raw(
                allowed
                    .into_iter()
                    .map(|s| MimeType::from(Cow::Owned(s)))
                    .collect::<Vec<_>>(),
            )
            .map(|(d, m)| (d, m.to_string()))?)
    }
}
