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


    setTimeout(() => {
        versionsManifest.val = Store.getGlobalManifestData() as unknown as VersionsManifest;
        console.log("Manifest loaded");
    }, 0)

    effect(() => {
        console.log(selectedVersion.val);
    }, [selectedVersion])


    const createLauncherRoot = async () => {
        Api.createLauncherRoot(rootPath.val)
            .then(json => { console.log(json); Store.setGlobalManifestData(json) })
            .catch(err => { console.log(err) })
    }

    const getVersionsManifest = async () => {
        Api.getVersionsManifest()
            .then(json => { 
                console.log(json); 
                Store.setGlobalManifestData(json); 
                console.log(Store.getGlobalManifestData());
                versionsManifest.val = Store.getGlobalManifestData() as unknown as VersionsManifest; 
            })
            .catch(err => { console.log(err) })
    }

    const handleVersionChange = async (version: Version) => {
        console.log(`id: ${version.id}, url: ${version.url}`);
        selectedVersion.val = version;
    }

    return (
        <>
            <input type="text" id="path" onInput={(e: { target: { value: string; }; }) => rootPath.val = e.target.value}></input>
            <button onClick={createLauncherRoot}>Create launcher root</button>
            <br /><br />
            <button onClick={getVersionsManifest}>Get Versions</button>
            <div class={css.selectionArea}>
                {versionsManifest.derive(val => val?.versions.map((version) => (
                    <button class={css.button} onClick={() => handleVersionChange(version)}>{version.id}</button>
                )))}
            </div>
        </>
    );
};

export default Instances;
