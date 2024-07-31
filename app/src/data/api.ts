import Constants from "@/data/constants";
import { RequestInstance } from "@/data/types";
import { fetch as tfetch, ResponseType } from "@tauri-apps/api/http";

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

                console.log(response);
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
      try {
        const response = await tfetch(
          Api.url + Constants.endpoints.versionsManifest,
          {
            method: "GET",
            responseType: ResponseType.JSON,
          },
        );

        if (response.status !== 200) {
          throw new Error(`HTTP Error, status: ${response.status}`);
        }

        return response.data as JSON;
      } catch (err) {
        throw new Error(`Error fetching versions manifest: ${err}`);
      }
    }

    public async requestVersionDownload(name: string, url: string, info: Record<string, string>): Promise<JSON> {
        let body = JSON.stringify({
            name,
            url,
            info
        })

        console.log(body);
        return new Promise<JSON>((res, rej) => {
            fetch(Api.url + Constants.endpoints.createInstance, {
                method: 'POST',
                body: body
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
