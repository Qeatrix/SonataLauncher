import { effect, ref } from 'hywer';
import Api from '@/data/api';
import Store from '@/data/store';
import { VersionsManifest, Version, RequestInstance } from '@/data/types';

import css from "./Instances.module.less";
import { SelectionArea, SelectionItem } from '@/ui/components/selectionArea/selectionArea';


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

        console.log(versionsManifest.val);
    }

    const requestVersionDownload = async () => {
        console.log(selectedVersion.val.url);

        const info = new Map<string, string>();
        info.set("${auth_player_name}", "Melicta");
        info.set("${version_name}", selectedVersion.val.id);
        info.set("${version_type}", selectedVersion.val.type);
        info.set("${user_type}", "legacy");
        info.set("${auth_uuid}", "99b3e9029022309dae725bb19e275ecb");
        info.set("${auth_access_token}", "[asdasd]");

        let infoObject: Record<string, string> = {};
        info.forEach((value, key) => {
            infoObject[key] = value;
        });
        
        Api.requestVersionDownload("asd", selectedVersion.val.url, infoObject)
            .then(json => { console.log(json) })
            .catch(err => { console.log(err) })
    }

    const handleVersionChange = async (version: Version, key: number) => {
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
            <br /><br />
            <input type="text" id="instance-name" />
            <div class={css.selectionArea}>
                <SelectionArea name="Versions">
                    <>
                        {versionsManifest.derive(val => val?.versions.map((version, key) => (
                            <SelectionItem
                                onClick={() => handleVersionChange(version, key)}
                                name={version.id}
                            >
                            </SelectionItem>
                        )))}
                    </>
                </SelectionArea>
            </div>
            <button onClick={requestVersionDownload}>Continue</button>
        </>
    );
};

export default Instances;