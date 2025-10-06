#![allow(unsafe_op_in_unsafe_fn)]

pub mod discord_bot_rs;
pub mod prelude;
pub mod python_wrappers;

use dotenv::dotenv;
use poise::serenity_prelude::GatewayIntents;
use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::discord_bot_rs::create_bot;


#[pyfunction]
fn start_bot_py<'py>(py: Python<'py>, token: String) -> PyResult<Bound<'py, PyAny>> {
    pyo3_asyncio_0_21::tokio::future_into_py(py, async move {
        let intents = GatewayIntents::all();
        let mut bot = create_bot(intents, token).await
            .map_err(|err| PyRuntimeError::new_err(format!("{}", err)))?;

        bot.start().await
            .map_err(|err|PyRuntimeError::new_err(format!("{}", err)))?;    

        Ok(())
    })
}

#[pymodule]
fn rust_src<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    dotenv().ok();
    
    m.add_function(wrap_pyfunction!(start_bot_py, m)?)?;
    Ok(())
}