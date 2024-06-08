import Constants from "@/data/constants";

class Api {
    public static readonly url = Constants.apiUrl;

    public async createLauncherRoot(path: string): Promise<JSON> {
        return new Promise<JSON>((res, rej) => {
            fetch(Api.url + Constants.endpoints.createRoot, {
                method: 'POST',
                body: JSON.stringify({
                    path: path
                })
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP Error, status: ${response.status}`);
                }

                return response.json();
            })
            .then(json => {
                res(json);
            })
            .catch(err => {
                rej(err);
            })
        })
    }

    public async getVersionsManifest(): Promise<JSON> {
        return new Promise<JSON>((res, rej) => {
            fetch(Api.url + Constants.endpoints.versionsManifest, {
                method: 'GET',
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP Error, status: ${response.status}`);
                }

                return response.json();
            })
            .then(json => {
                res(json);
            })
            .catch(err => {
                rej(err);
            })
        })
    }

    public async getExampleData(): Promise<JSON> {
        return new Promise<JSON>((res, rej) => {
            fetch(Api.url + Constants.endpoints.exampleData, {
                method: 'POST',
                body: JSON.stringify({
                    name: "Denis",
                    legs: 2,
                })
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP Error, status: ${response.status}`);
                }

                return response.json();
            })
            .then(json => {
                res(json);
            })
            .catch(err => {
                rej(err);
            })
        })
    }
}

const ApiInstance = new Api;
export default ApiInstance;