/*
* @author Jakob Grätz
* @description Checks if a given API key is a valid API key.
*/
pub fn check_api_key(api_key: String) -> bool {
    if api_key == "abc123" {
        return true;
    } else {
        return false;
    }
}