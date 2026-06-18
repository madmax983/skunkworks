import re

with open("experiments/fabric-limb/src/physics.rs", "r") as f:
    content = f.read()

new_tests = """

    #[test]
    fn should_not_panic_when_joints_cleared() {
        let mut arm = Arm::new((0.0, 0.0), vec![10.0, 10.0]);
        arm.joints.clear();
        arm.solve((5.0, 5.0)); // Should just return
    }

    #[test]
    fn should_not_panic_when_joints_truncated_unreachable() {
        let mut arm = Arm::new((0.0, 0.0), vec![10.0, 10.0]);
        arm.joints.pop();
        arm.solve((50.0, 50.0)); // Unreachable target branch
    }

    #[test]
    fn should_not_panic_when_joints_truncated_reachable() {
        let mut arm = Arm::new((0.0, 0.0), vec![10.0, 10.0]);
        arm.joints.pop();
        arm.solve((5.0, 5.0)); // Reachable target branch
    }
"""

content = re.sub(r'    \}\n\}$', '    }' + new_tests + '}', content)

with open("experiments/fabric-limb/src/physics.rs", "w") as f:
    f.write(content)
