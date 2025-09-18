use super::*;

#[test]
fn test_rotation_orders() {
    // X rotation should have order 4 (x^4 = identity)
    assert_eq!(X * X * X * X, CubeRotation::ID);
    assert_ne!(X * X * X, CubeRotation::ID);

    // Y rotation should have order 4
    assert_eq!(Y * Y * Y * Y, CubeRotation::ID);
    assert_ne!(Y * Y * Y, CubeRotation::ID);

    // Z rotation should have order 4
    assert_eq!(Z * Z * Z * Z, CubeRotation::ID);
    assert_ne!(Z * Z * Z, CubeRotation::ID);
}

#[test]
fn test_xyz_order() {
    // Let's find what order X*Y*Z actually has
    let xyz = X * Y * Z;
    let mut power = xyz;
    let mut order = 1;

    while power != CubeRotation::ID && order <= 24 {
        power = power * xyz;
        order += 1;
    }

    println!("XYZ = {:?}", xyz);
    println!("Order of XYZ: {}", order);

    // Also check XZY
    let xzy = X * Z * Y;
    let mut power_xzy = xzy;
    let mut order_xzy = 1;
    while power_xzy != CubeRotation::ID && order_xzy <= 24 {
        power_xzy = power_xzy * xzy;
        order_xzy += 1;
    }
    println!("XZY = {:?}", xzy);
    println!("Order of XZY: {}", order_xzy);

    assert!(order <= 24, "XYZ should have finite order");
}

#[test]
fn test_group_closure() {
    // The group should be closed under multiplication
    // Test that xyz has finite order
    let xyz = X * Y * Z;
    let mut power = xyz;
    let mut found_identity = false;

    for _ in 1..25 { // At most 24 elements in the group
        if power == CubeRotation::ID {
            found_identity = true;
            break;
        }
        power = power * xyz;
    }
    assert!(found_identity, "xyz should have finite order");
}

#[test]
fn test_associativity() {
    // Check associativity: (X*Y)*Z = X*(Y*Z)
    assert_eq!((X * Y) * Z, X * (Y * Z));
    assert_eq!((Y * Z) * X, Y * (Z * X));
    assert_eq!((Z * X) * Y, Z * (X * Y));
}

#[test]
fn test_identity_constants() {
    // All identity constants should be equal
    assert_eq!(CubeRotation::ID, CubeRotation::ONE);
    assert_eq!(CubeRotation::ONE, CubeRotation::E);

    // Identity should be a left and right identity
    assert_eq!(CubeRotation::ID * X, X);
    assert_eq!(X * CubeRotation::ID, X);
    assert_eq!(CubeRotation::E * Y, Y);
    assert_eq!(Y * CubeRotation::E, Y);
}

#[test]
fn test_rotation_inverses() {
    // Verify all X3 = X*X*X by computing manually
    let x_cubed = X * X * X;
    let y_cubed = Y * Y * Y;
    let z_cubed = Z * Z * Z;

    println!("X*X*X = {:?}, X3 = {:?}", x_cubed, X3);
    println!("Y*Y*Y = {:?}, Y3 = {:?}", y_cubed, Y3);
    println!("Z*Z*Z = {:?}, Z3 = {:?}", z_cubed, Z3);

    assert_eq!(x_cubed, X3);
    assert_eq!(y_cubed, Y3);
    assert_eq!(z_cubed, Z3);

    // Test that X3 is the inverse of X
    assert_eq!(X * X3, CubeRotation::ID);
    assert_eq!(X3 * X, CubeRotation::ID);

    // Test that Y3 is the inverse of Y
    assert_eq!(Y * Y3, CubeRotation::ID);
    assert_eq!(Y3 * Y, CubeRotation::ID);

    // Test that Z3 is the inverse of Z
    assert_eq!(Z * Z3, CubeRotation::ID);
    assert_eq!(Z3 * Z, CubeRotation::ID);
}

#[test]
fn test_rotation_squares() {
    // Test that X2 = X * X
    assert_eq!(X * X, X2);

    // Test that Y2 = Y * Y
    assert_eq!(Y * Y, Y2);

    // Test that Z2 = Z * Z
    assert_eq!(Z * Z, Z2);

    // Test that X2 is its own inverse
    assert_eq!(X2 * X2, CubeRotation::ID);

    // Test that Y2 is its own inverse
    assert_eq!(Y2 * Y2, CubeRotation::ID);

    // Test that Z2 is its own inverse
    assert_eq!(Z2 * Z2, CubeRotation::ID);
}

#[test]
fn test_face_perm_conversion() {
    use Face::*;

    // Test X rotation face permutation
    let x_face_perm: FacePerm = X.into();

    // X rotation (90° around X-axis) moves:
    // Up -> Back, Down -> Front, Left -> Left, Right -> Right, Front -> Up, Back -> Down
    assert_eq!(x_face_perm[Up], Back);
    assert_eq!(x_face_perm[Down], Front);
    assert_eq!(x_face_perm[Left], Left);
    assert_eq!(x_face_perm[Right], Right);
    assert_eq!(x_face_perm[Front], Up);
    assert_eq!(x_face_perm[Back], Down);

    // Test Y rotation face permutation
    let y_face_perm: FacePerm = Y.into();

    // Y rotation (90° around Y-axis) moves:
    // Up -> Up, Down -> Down, Left -> Back, Right -> Front, Front -> Left, Back -> Right
    assert_eq!(y_face_perm[Up], Up);
    assert_eq!(y_face_perm[Down], Down);
    assert_eq!(y_face_perm[Left], Back);
    assert_eq!(y_face_perm[Right], Front);
    assert_eq!(y_face_perm[Front], Left);
    assert_eq!(y_face_perm[Back], Right);

    // Test Z rotation face permutation
    let z_face_perm: FacePerm = Z.into();

    // Z rotation (90° around Z-axis) moves:
    // Up -> Right, Down -> Left, Left -> Up, Right -> Down, Front -> Front, Back -> Back
    assert_eq!(z_face_perm[Up], Right);
    assert_eq!(z_face_perm[Down], Left);
    assert_eq!(z_face_perm[Left], Up);
    assert_eq!(z_face_perm[Right], Down);
    assert_eq!(z_face_perm[Front], Front);
    assert_eq!(z_face_perm[Back], Back);
}

#[test]
fn test_identity_face_perm() {
    use Face::*;

    // Identity rotation should map each face to itself
    let id_face_perm: FacePerm = CubeRotation::ID.into();

    assert_eq!(id_face_perm[Up], Up);
    assert_eq!(id_face_perm[Down], Down);
    assert_eq!(id_face_perm[Left], Left);
    assert_eq!(id_face_perm[Right], Right);
    assert_eq!(id_face_perm[Front], Front);
    assert_eq!(id_face_perm[Back], Back);
}