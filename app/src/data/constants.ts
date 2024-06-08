class Constants {
    public readonly apiUrl = "http://127.0.0.1:8080/";

    public readonly endpoints = {
        exampleData: "orders/shoes",
        createRoot: "init/root",
        versionsManifest: "instance/download_versions"
    }
}

const ConstantsInstance = new Constants;
export default ConstantsInstance;