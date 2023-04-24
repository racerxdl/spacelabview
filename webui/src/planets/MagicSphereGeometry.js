/*
    Magic Sphere = Deformed Cube to look like a sphere
*/

import * as THREE from 'three';

let cachedMagicSphere;

function magicSphereGeometry(radius, segments) {
    if (!cachedMagicSphere) {
        console.log(`Magic Sphere not cached. Caching it...`)
        cachedMagicSphere = new THREE.BoxGeometry(1, 1, 1, segments, segments, segments);
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
        const uvAttribute = cachedMagicSphere.getAttribute('uv');

        const midx = (idx) => {
            for (let i = 0; i < cachedMagicSphere.groups.count; i++) {
                const group = cachedMagicSphere.groups[i];
                if (group.start <= idx && idx - group.start < group.count) {
                    return group.materialIndex;
                }
            }
            return 0;
        }

        for (let i = 0; i < cachedMagicSphere.index.count; i += 3) {
            const materialIndex = midx(i);
            for (let j = 0; j < 3; j++) {
                const faceIdx = cachedMagicSphere.index[i * 3 + j];
                uv.fromBufferAttribute(uvAttribute, faceIdx);
                uv.multiply(repeat).add(offsets[materialIndex]);
                uvAttribute.setXY(faceIdx, uv.x, uv.y);
            }
        }
        uvAttribute.needsUpdate = true;
    }
    const geometry = cachedMagicSphere.clone();
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