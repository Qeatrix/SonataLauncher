import { For } from "hywer/x/html"
import { SelectionItem } from "./selectionArea"
import { Reactive } from "hywer/jsx-runtime";
import Store from '@/data/store';
import css from './selectionArea.module.less';
import { VersionsManifest } from "@/data/types";


interface SearchProps {
    query: string
    onSelect: (id: string, url: string) => any;
    selectedValue: Reactive<string>
}

export default function Search({query, onSelect, selectedValue}: SearchProps) {
    const manifest = Store.getGlobalManifestData() as unknown as VersionsManifest

    function checkVersion(name: string) {
        const escapedWord = query.trim().replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
        const regex = new RegExp('^(?<!\\w)(?:'+escapedWord+')\\w*', 'iu');

        let matches = name.match(regex);

        return matches
    }

    const result = manifest.versions.filter(version =>
        checkVersion(version.id)
    );


    return (
        <For in={result}>
            {(item, i) => {
                return <SelectionItem
                    name={item.id}
                    onClick={() => onSelect(item.id, item.url)}
                    selected={selectedValue.val == item.id}
                />
            }}
        </For>
    )
}