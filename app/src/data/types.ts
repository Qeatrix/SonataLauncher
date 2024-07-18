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


interface WS {
    message_id: string,
    message_type: string,
    timestamp: string,
}

export interface InfoMessage extends WS {
    message: string;
}

export interface ErrorMessage extends WS {
    details: {
        reason: string,
        suggestions: string[]
    }
}


interface ProgressTarget_File {
    status: string,
    name: string,
    size_bytes: number,
}

interface ProgressTarget_Dir {
    path: String,
}

export enum ProgressStatuses {
    PENDING = "PENDING",
    INPROGRESS = "INPROGRESS",
    COMPLETED = "COMPLETED",
    FAILED = "FAILED"
}

export interface ProgressMessage extends WS {
    data: {
        stage: ProgressStatuses,
        determinable: boolean,
        progress: number,
        max: number,
        status: string,
        target_type: string,
        target: ProgressTarget_File | ProgressTarget_Dir,
    }
}

export interface ProgressTargetsList extends WS {
    ids_list: string[]
}

export interface ProgressMessageFinish extends WS {
    data: {
        stage: string,
        status: string,
    }
}