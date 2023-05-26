
import * as THREE from 'three';

const GenerateRing = (innerRadius, outerRadius, segmentsY) => {
  const avgRadius = (innerRadius + outerRadius) / 2;
  const range = outerRadius - innerRadius;
  const geometry = new THREE.RingGeometry(innerRadius, outerRadius, segmentsY, 3);
  var pos = geometry.attributes.position;
  var v3 = new THREE.Vector3();
  // UV mapping is a bit different for our textures
  for (let i = 0; i < pos.count; i++) {
    v3.fromBufferAttribute(pos, i);
    const angle = v3.angleTo(new THREE.Vector3(1, 0, 0)) / (Math.PI * 2);
    const l = (v3.length() - innerRadius) / range;
    const ratio = Math.abs((l * 2) - 1);
    geometry.attributes.uv.setXY(i, angle, ratio);
  }
  // const extrudedGeometry = new THREE.ExtrudeGeometry(geometry, {
  //   depth: 8, bevelEnabled: true, bevelSegments: 2, steps: 2, bevelSize: 1, bevelThickness: 1
  // });
  // return extrudedGeometry;
  return geometry;
}
export {
  GenerateRing,
}