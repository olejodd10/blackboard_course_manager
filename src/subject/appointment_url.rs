pub struct AppointmentUrlFormatString(pub Vec<String>);

impl AppointmentUrlFormatString {
    pub fn appointment_url(&self, appointment_number: usize) -> String {
        String::from(self.0.join(&format!("{}", appointment_number)))
    }
}