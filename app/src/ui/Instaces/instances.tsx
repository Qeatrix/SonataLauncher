import { effect, ref } from 'hywer';
import Api from '@/data/api';
import Store from '@/data/store';
import { VersionsManifest, Version } from '@/data/types';

import css from "./Instances.module.less";


const Instances = () => {
    const rootPath = ref("");
    const versionsManifest = ref<VersionsManifest>({
        latest: {
            release: '',
            snapshot: ''
        },
        versions: []
    });

    const selectedVersion = ref<Version>({
        complianceLevel: 0,
        id: '',
        releaseTime: '',
        sha1: '',
        time: '',
        type: '',
        url: ''
    });

    const selectedVersionNumber = ref<number>(0);


    setTimeout(() => {
        versionsManifest.val = Store.getGlobalManifestData() as unknown as VersionsManifest;
        console.log("Manifest loaded");
    }, 0)

    effect(() => {
        console.log(selectedVersion.val);
    }, [selectedVersion])


    const createLauncherRoot = async () => {
        Api.createLauncherRoot(rootPath.val)
            .then(json => { console.log(json) })
            .catch(err => { console.log(err) })
    }

    const getVersionsManifest = async () => {
        Api.getVersionsManifest()
            .then(json => { 
                Store.setGlobalManifestData(json); 
                versionsManifest.val = json as unknown as VersionsManifest; 
            })
            .catch(err => { console.log(err) })
    }

    const requestVersionDownload = async () => {
        Api.requestVersionDownload(selectedVersion.val.id, selectedVersion.val.id, selectedVersion.val.url)
            .then(json => { console.log(json) })
            .catch(err => { console.log(err) })
    }

    const handleVersionChange = async (e: PointerEvent, version: Version, key: number) => {
        selectedVersion.val = version;
        Store.setVersionManifestData(version.id, version);
        selectedVersionNumber.val = key;
    }

    return (
        <>
            <input type="text" id="path" onInput={(e: { target: { value: string; }; }) => rootPath.val = e.target.value}></input>
            <button onClick={createLauncherRoot}>Create launcher root</button>
            <br /><br />
            <button onClick={getVersionsManifest}>Get Versions</button>
            <div class={css.selectionArea}>
                {versionsManifest.derive(val => val?.versions.map((version, key) => (
                    <button
                        class={selectedVersionNumber.derive(val => `${css.button} ${val === key ? css.selectedButton : ''}`)}
                        key={key}
                        onClick={(e: PointerEvent) => handleVersionChange(e, version, key)}
                    >
                        {version.id}
                    </button>
                )))}
            </div>
            <button onClick={requestVersionDownload}>Continue</button>
        </>
    );
};

export default Instances;
