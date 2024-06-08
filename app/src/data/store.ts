class Store {
    private static globalVersionsManifest = {
        key: 'versionsManifest',
        data: {}
    };

    private static versionManifest = {}

    public getGlobalManifestData() {
        const storedData = localStorage.getItem(Store.globalVersionsManifest.key);

        if (storedData) {
            return JSON.parse(storedData);
        } else {
            return Store.globalVersionsManifest.data;
        }
    }

    public setGlobalManifestData(data: JSON) {
        localStorage.setItem(Store.globalVersionsManifest.key, JSON.stringify(data));
    }

    public getVersionManifestData(version: string) {
        const storedData = localStorage.getItem(version);

        if (storedData) {
            return JSON.parse(storedData);
        } else {
            return {};
        }
    }
}

const StoreInstance = new Store;
export default StoreInstance;