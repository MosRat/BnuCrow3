/*
 * Copyright (c) 2024. MosRat
 * All rights reserved.
 *
 * Project: BnuCrow3
 * File Name: index.ts
 * Created Date: 2024/10/30 11:20
 * Author: MosRat (work@whl.moe)
 * Description:
 */

import {load} from '@tauri-apps/plugin-store';
import {appDataDir} from '@tauri-apps/api/path'


export const store = await load(  (await appDataDir()) + '/store.bin', {
    // we can save automatically after each store modification
    // @ts-ignore
    autoSave: 100,
});