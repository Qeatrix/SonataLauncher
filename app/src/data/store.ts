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

    private activeIDs: string[] = [];


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


    public makeId(length: number) {
        let result = '';
        const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
        const charactersLength = characters.length;
        let counter = 0;

        while (counter < length) {
          result += characters.charAt(Math.floor(Math.random() * charactersLength));
          counter += 1;
        }

        if (this.activeIDs.includes(result)) {
            result = this.makeId(length);
        }

        this.activeIDs.push(result);
        return result;
    }

    public deactiveId(id: string) {
        this.activeIDs = this.activeIDs.filter((i) => i !== id);
    }
}

const StoreInstance = new Store;
export default StoreInstance;