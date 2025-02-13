# bye_orbslam3_rs
使用纯Rust实现的ORB-SLAM3.

- 状态: **早期阶段**.不建议用于实际生产用途.

## 简介
ORB-SLAM3 是一个能够进行 **视觉、视觉-惯性和多地图 SLAM** 的实时 SLAM 库，支持 **单目、立体和 RGB-D** 相机，使用 **针孔和鱼眼** 镜头模型。在所有传感器配置中，ORB-SLAM3 的鲁棒性与文献中的系统相当，并且精度显著更高。 

## 参考资料
```bibtex
@article{ORBSLAM3_TRO,
    title={{ORB-SLAM3}: An Accurate Open-Source Library for Visual, Visual-Inertial 
              and Multi-Map {SLAM}},
    author={Campos, Carlos AND Elvira, Richard AND G\´omez, Juan J. AND Montiel, 
            Jos\'e M. M. AND Tard\'os, Juan D.},
    journal={IEEE Transactions on Robotics}, 
    volume={37},
    number={6},
    pages={1874-1890},
    year={2021}
}
```
