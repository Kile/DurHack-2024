use kube::{Client, Api};
use serde_json::json;
use k8s_openapi::api::core::v1::Pod;

pub async fn create_pod(client: Client, name: &str) -> Result<(), kube::Error> {
    let pods: Api<Pod> = Api::namespaced(client, "default");
    let pod_spec = json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": name },
        "spec": { "containers": [{ "name": "stock-api", "image": "your-docker-image" }] }
    });

    let pod: Pod = serde_json::from_value(pod_spec).expect("Failed to deserialize Pod");

    pods.create(&Default::default(), &pod).await?;
    Ok(())
}