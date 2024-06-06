import { useEffect, useState } from 'react';

import { downloadMainManifest, downloadVersionManifest, initRoot } from './ManifestManagment';

import './Instances.css';

function Instaces() {
    const [versions, setVersions] = useState(null);
    const [versionsError, setVersionsError] = useState(null);
    const [selectedVersion, setSelectedVersion] = useState(({ "version": "none" }));
    const [downloadedVersion, setDownloadedVersion] = useState(null);
    const [downloadedVersionError, setDownloadedVersionError] = useState(null);

    useEffect(() => {

    }, [])

    const getMainManifest = () => {
        const headerState = document.getElementById('header-state');
        downloadMainManifest(headerState, versions, setVersions, setVersionsError);
    }

    const getVersionManifest = () => {
        const pState = document.getElementById('p-state');
        const url = "https://piston-meta.mojang.com/v1/packages/111890b5a8c2fee9b77036f9f377b33df42c718a/1.20.6.json";
        downloadVersionManifest(selectedVersion, url, downloadedVersion, setDownloadedVersion, setDownloadedVersionError, pState);
    }

    const handleInitRoot = () => {
        initRoot();
    }

    const handleChangeSelectedValue = (e) => {
        setSelectedVersion(
            ({
                "version": e.target.value,
                "index": parseInt(e.target.dataset.index)
            })
        );
    }

    return (
        <>
            <h1 id="header-state" className='instances-heading'>Instances Manager</h1>
            <button onClick={getMainManifest}>Download Minecraft Manifest</button>
            {
                versions && (
                    <div>
                        <h2>Selector Available:</h2>
                        <select value={selectedVersion.version} onChange={handleChangeSelectedValue}>
                            <option value="ver">Select the version</option>
                            {versions.versions.map((item, index) => (
                                <option key={index} value={item.id}>{item.id}</option>
                            ))}
                        </select>
                        <p>Selected Version: {selectedVersion.version}</p>
                        {
                            selectedVersion !== null && (
                                <>
                                    <button onClick={getVersionManifest}>Download {selectedVersion.version} Manifest</button>
                                    <p id="p-state">{selectedVersion.version} is not downloaded yet</p>
                                </>
                            )
                        }
                        {
                            downloadedVersion !== null && (
                                <>
                                    <p>Downloaded Data:</p>
                                </>
                            )
                        }
                    </div>
                )
            }
            {versionsError && (
                <div className="">
                    <h2>Error While Downloading:</h2>
                    <pre>{versionsError.toString()}</pre>
                </div>
            )}
            <br />
            <br />
            <input type="text" placeholder="Launcher Folder" id="dir" />
            <button onClick={handleInitRoot}>Apply</button>
        </>
    )
}

export default Instaces;