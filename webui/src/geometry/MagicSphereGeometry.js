/*
    Magic Sphere = Deformed Cube to look like a sphere
*/

import * as THREE from 'three';
import { mergeVertices } from './BufferedGeometryUtils';

let cachedMagicSpheres = {};

function magicSphereGeometry(radius, segments) {
    if (!cachedMagicSpheres[segments]) {
        console.log(`Magic Sphere (${segments}) not cached. Caching it...`)
        cachedMagicSpheres[segments] = new THREE.BoxGeometry(1, 1, 1, segments, segments, segments);
        // clear normal from vertices
        cachedMagicSpheres[segments].deleteAttribute('normal');
        cachedMagicSpheres[segments] = mergeVertices(cachedMagicSpheres[segments]);

        // recreate normal
        cachedMagicSpheres[segments].computeVertexNormals();

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
        const uvAttribute = cachedMagicSpheres[segments].getAttribute('uv');

        const midx = (idx) => {
            for (let i = 0; i < cachedMagicSpheres[segments].groups.count; i++) {
                const group = cachedMagicSpheres[segments].groups[i];
                if (group.start <= idx && idx - group.start < group.count) {
                    return group.materialIndex;
                }
            }
            return 0;
        }

        for (let i = 0; i < cachedMagicSpheres[segments].index.count; i += 3) {
            const materialIndex = midx(i);
            for (let j = 0; j < 3; j++) {
                const faceIdx = cachedMagicSpheres[segments].index[i * 3 + j];
                uv.fromBufferAttribute(uvAttribute, faceIdx);
                uv.multiply(repeat).add(offsets[materialIndex]);
                uvAttribute.setXY(faceIdx, uv.x, uv.y);
            }
        }
        uvAttribute.needsUpdate = true;
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