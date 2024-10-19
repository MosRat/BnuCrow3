import {createStore} from '@tauri-apps/plugin-store';
import {appDataDir} from '@tauri-apps/api/path'


export const store = await createStore(  (await appDataDir()) + '/store.bin', {
    // we can save automatically after each store modification
    // @ts-ignore
    autoSave: 100,
});