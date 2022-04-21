use vgs::redact_via_reverse_proxy;

fn main() {
    let original_data = "Jacob Zuma";
    let reverse_http_proxy_host = "company.environment.verygoodproxy.com";

    redact_via_reverse_proxy(original_data, reverse_http_proxy_host);
}
