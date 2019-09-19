export function get_error_name(error) {
  return error.get_name();
}

export function get_error_desc(error) {
  return error.description();
}

export function get_error_cause(error) {
  return error.cause();
}
