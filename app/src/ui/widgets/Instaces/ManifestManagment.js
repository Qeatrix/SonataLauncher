import { invoke } from '@tauri-apps/api/tauri';

const downloadMainManifest = async(headerState, versions, setVersions, setError) => {

    headerState.innerHTML = "Downloading manifest...";

    if (versions === null) {
        try
        {
            const result = await invoke('download_main_manifest');
            setVersions(result);
            console.log(result.versions);
        }
        catch(e)
        {
            setError(e);
        }
    }

    headerState.innerHTML = "Downloading Completed";
};

const downloadVersionManifest = async(version, url, downloadedVersion, setDownloadedVersion, setDownloadedVersionError, pState) => {
    try
    {
        const result = await invoke('download_version_manifest', { url: url });
        setDownloadedVersion(({
            [version]: true
        }));

        console.log(result);
        pState.innerHTML = version + " downloaded";
    }
    catch(e)
    {
        setDownloadedVersionError(e);
    }
}

const initRoot = async() => {
    try
    {
        const result = await invoke('init_root');
        console.log(result);
    }
    catch(e)
    {
        console.log(e)
    }
}

export { downloadMainManifest, downloadVersionManifest, initRoot };