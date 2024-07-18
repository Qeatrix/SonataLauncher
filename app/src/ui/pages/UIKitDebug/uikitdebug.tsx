import { effect, ref } from 'hywer/jsx-runtime';
import Api from '@/data/api';
import Store from '@/data/store';
import Button from '@/ui/components/buttons/buttons';
import Input from '@/ui/components/input/input';
import { SelectionArea, SelectionItem } from '@/ui/components/selectionArea/selectionArea';
import { ContentStack, FlexBox, Window, WindowControls } from '@/ui/components/window/window';
import { ProgressDisplay, ProgressItem } from '@/ui/widgets/progressDisplay/ProgressDisplay';
import { VersionsManifest, Version, ProgressMessage, ProgressTargetsList } from '@/data/types';
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

    const selectedLoaderNumber = ref(0);
    const Loaders = [
            {
                "name": "Vanilla",
            },
            {
                "name": "Fabric",
            }
    ];

    const contentStackIndex = ref<number>(0);

    const progressItems = ref<ProgressTargetsList>({
        message_id: '',
        timestamp: '',
        ids_list: []
    });


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
        console.log(selectedVersionId.val);
    }

    console.log(versionsManifest.val);

    const check = () => {
        console.log(versionsManifest.val?.versions);
    }

    const handleLoaderChange = (key: number) => {
        selectedLoaderNumber.val = key;
    }

    const previousWindow = () => {
        contentStackIndex.val--;
    }

    const nextWindow = () => {
        contentStackIndex.val++;
    }

    return (
        <>
            <p>asdasd</p>
            <button onClick={getVersionsManifest}>get versions</button>
            <Window name="Instance Creation">
                <ContentStack showIndex={contentStackIndex}>
                    <div>
                        <FlexBox>
                            <Input id="name" name="Name" />
                            <>
                                <div></div>
                            </>
                        </FlexBox>
                        <FlexBox>
                            <Input id="tags" name="Tags" />
                            <>
                                <div></div>
                            </>
                        </FlexBox>
                        <FlexBox>
                            <SelectionArea selectedValue={selectedVersionId} onValueChange={handleVersionChange} name="Versions" searchBar={true}>
                                <div>
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
                                </div>
                            </SelectionArea>
                            <SelectionArea selectedValue={selectedVersionId} onValueChange={handleVersionChange} name="Loader">
                                <div>
                                    <For in={Loaders}>
                                        {(loader, i) => {
                                            return <SelectionItem
                                                name={loader.name}
                                                onClick={() => handleLoaderChange(i)}
                                                selected={selectedLoaderNumber.derive(val => val == i)}
                                            />
                                        }}
                                    </For>
                                </div>
                            </SelectionArea>
                        </FlexBox>
                    </div>
                    <div>
                        <ProgressDisplay>
                            <div>
                                {progressItems.derive(val => {
                                    return <For in={val.ids_list}>
                                        {(stage) => {
                                            if (stage) {
                                                console.log("STAGEE!!!!");
                                                return <ProgressItem name={stage} />
                                            }
                                        }}
                                    </For>
                                })}
                            </div>
                        </ProgressDisplay>
                    </div>
                    <div>
                        <FlexBox>
                            <Input id="asd123" name="Name" />
                            <>
                                <div></div>
                            </>
                        </FlexBox>
                        <FlexBox>
                            <Input id="dkfjgh" name="Tags" />
                            <>
                                <div></div>
                            </>
                        </FlexBox>
                    </div>
                </ContentStack>
                <WindowControls>
                    <Button text="Cancel" onClick={previousWindow} />
                    <Button text="Continue" primary={true} onClick={nextWindow} />
                </WindowControls>
            </Window>
        </>
    )
}

export default page;