struct TextInput {
    label: &'static str,
    value: String,
    focused: bool,
}

enum FormType {
    Artist { name: TextInput },
}
