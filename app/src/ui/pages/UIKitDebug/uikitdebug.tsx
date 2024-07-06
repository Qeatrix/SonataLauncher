import { effect, ref } from 'hywer/jsx-runtime';
import Api from '@/data/api';
import Store from '@/data/store';
import Button from '@/ui/components/buttons/buttons';
import Input from '@/ui/components/input/input';
import { SelectionArea, SelectionItem } from '@/ui/components/selectionArea/selectionArea';
import { FlexBox, Window, WindowControls } from '@/ui/components/window/window';
import { VersionsManifest, Version } from '@/data/types';
import { For } from 'hywer/x/html';

function page() {
    const versionsManifest = ref<VersionsManifest>({
        latest: {
            release: '',
            snapshot: ''
        },
        versions: []
    });

    const selectedVersionId = ref<string>("");

    const selectedVersion = ref<Version>({
        complianceLevel: 0,
        id: '',
        releaseTime: '',
        sha1: '',
        time: '',
        type: '',
        url: ''
    });

    const selectedVersionNumber = ref<string>("");

    const selectedLoaderNumber = ref(0);
    const Loaders = [
            {
                "name": "Vanilla",
            },
            {
                "name": "Fabric",
            }
    ];


    setTimeout(() => {
        versionsManifest.val = Store.getGlobalManifestData() as unknown as VersionsManifest;
        console.log("Manifest loaded");
    }, 0)

    effect(() => {
        console.log(selectedVersion.val);
    }, [selectedVersion])


    const getVersionsManifest = async () => {
        Api.getVersionsManifest()
            .then(json => { 
                Store.setGlobalManifestData(json); 
                versionsManifest.val = json as unknown as VersionsManifest; 
            })
            .catch(err => { console.log(err) })

        console.log(versionsManifest.val);
    }

    const handleVersionChange = async (id: string) => {
        //selectedVersion.val = version;
        //Store.setVersionManifestData(version.id, version);
        selectedVersionId.val = id;
        console.log(123);
    }

    console.log(versionsManifest.val);

    const check = () => {
        console.log(versionsManifest.val?.versions);
    }

    const handleLoaderChange = (key: number) => {
        selectedLoaderNumber.val = key;
    }

    return (
        <>
            <p>asdasd</p>
            <button onClick={getVersionsManifest}>get versions</button>
            <Window name="Instance Creation">
                <FlexBox>
                    <Input name="Name" />
                    <>
                        <div></div>
                    </>
                </FlexBox>
                <FlexBox>
                    <Input name="Tags" />
                    <>
                        <div></div>
                    </>
                </FlexBox>
                <FlexBox>
                    <SelectionArea name="Versions">
                        <>
                            {/* {versionsManifest.derive(val => val?.versions.map((version, key) => (
                                <SelectionItem
                                    name={version.id}
                                    onClick={() => handleVersionChange(version, key)}
                                    className={selectedVersionNumber.derive(val => `${val === key ? 'selected' : ''}`)}
                                />
                            )))} */}
                            {versionsManifest.derive(val => {
                                return <For in={val.versions}>
                                    {(item, i) => {
                                        return <SelectionItem
                                            name={item.id}
                                            onClick={() => handleVersionChange(item.id)}
                                            selected={selectedVersionId.derive(val => val == item.id)}
                                        />
                                    }}
                                </For>
                            })}
                        </>
                    </SelectionArea>
                    <SelectionArea name="Loader">
                        <>
                            {Loaders.map((loader, key) => (
                                <SelectionItem
                                    name={loader.name}
                                    onClick={() => handleLoaderChange(key)}
                                    selected={selectedVersionId.derive(val => val == loader.name)}
                                />
                            ))}
                        </>
                    </SelectionArea>
                </FlexBox>
                <WindowControls>
                    <Button text="Cancel" />
                    <Button text="Continue" primary={true} />
                </WindowControls>
            </Window>
        </>
    )
}

export default page;