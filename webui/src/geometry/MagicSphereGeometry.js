/*
    Magic Sphere = Deformed Cube to look like a sphere
*/

import * as THREE from 'three';
import { mergeVertices } from './BufferedGeometryUtils';

let cachedMagicSpheres = {};

function magicSphereGeometry(radius, segments) {
    if (!cachedMagicSpheres[segments]) {
        console.log(`Magic Sphere (${segments}) not cached. Caching it...`)
        const geometry = new THREE.BoxGeometry(1, 1, 1, segments, segments, segments);
        const uv = new THREE.Vector2();
        const repeat = new THREE.Vector2(1 / 3, 1 / 2);
        const offsets = [
            new THREE.Vector2(0, 0),
            new THREE.Vector2(0, 1 / 2),
            new THREE.Vector2(1 / 3, 0),
            new THREE.Vector2(1 / 3, 1 / 2),
            new THREE.Vector2(2 / 3, 0),
            new THREE.Vector2(2 / 3, 1 / 2)
        ];
        const uvAttribute = geometry.getAttribute('uv');

        // Precompute material indices per face
        const materialIndices = [];
        for (let i = 0; i < geometry.groups.length; i++) {
            const group = geometry.groups[i];
            const materialIndex = group.materialIndex;
            for (let j = group.start; j < group.start + group.count; j += 3) {
                materialIndices.push(materialIndex);
            }
        }

        for (let i = 0; i < geometry.index.count; i += 3) {
            const materialIndex = materialIndices[i / 3];
            for (let j = 0; j < 3; j++) {
                const faceIdx = geometry.index[i * 3 + j];
                uv.fromBufferAttribute(uvAttribute, faceIdx);
                uv.multiply(repeat).add(offsets[materialIndex]);
                uvAttribute.setXY(faceIdx, uv.x, uv.y);
            }
        }

        uvAttribute.needsUpdate = true;
        cachedMagicSpheres[segments] = geometry;
        console.log(`Cached Magic Sphere (${segments})`);
    }

    const geometry = cachedMagicSpheres[segments].clone();
    const vertex = new THREE.Vector3();

    // texture is a collage; set offset/repeat per material index
    const positionAttribute = geometry.getAttribute('position');
    const normalAttribute = geometry.getAttribute('normal');

    // morph box into a sphere
    for (let i = 0; i < positionAttribute.count; i++) {
        vertex.fromBufferAttribute(positionAttribute, i);
        vertex.normalize();
        normalAttribute.setXYZ(i, vertex.x, vertex.y, vertex.z);
        vertex.multiplyScalar(radius);
        positionAttribute.setXYZ(i, vertex.x, vertex.y, vertex.z);
    }

    positionAttribute.needsUpdate = true;
    normalAttribute.needsUpdate = true;

    return geometry;
}

export {
    magicSphereGeometry
};