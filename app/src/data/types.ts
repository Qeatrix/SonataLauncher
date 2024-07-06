export interface Version {
    complianceLevel: number;
    id: string;
    releaseTime: string;
    sha1: string;
    time: string;
    type: string;
    url: string;
}

export interface VersionsManifest {
    latest: {
        release: string;
        snapshot: string;
    };

    versions: Version[];
}

export interface RequestInstance {
    name: string,
    url: string,
    info: Record<string, string>;
}

export interface Loader {
    name: string,
}