import { Version } from "@/data/types";

class Store {
    private static globalVersionsManifest = {
        key: 'globalVersionsManifest',
        data: {}
    };

    private static selectedVersionManifest = {
        key: 'selectedVersionManifest',
        data: {}
    }

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
        const storedData = localStorage.getItem(Store.selectedVersionManifest.key);

        if (storedData) {
            return JSON.parse(storedData);
        } else {
            return Store.selectedVersionManifest.data;
        }
    }

    public setVersionManifestData(version: string, data: Version) {
        localStorage.setItem(Store.selectedVersionManifest.key, JSON.stringify(data));
    }
}

const StoreInstance = new Store;
export default StoreInstance;