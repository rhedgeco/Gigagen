using UnityEngine;

public class FreeCam : MonoBehaviour
{
    private const float RotationScaler = 100;

    [SerializeField] private float flySpeed = 10;
    [SerializeField] private float shiftMultiplier = 2;
    [SerializeField] private float rotationSpeed = 5;

    private bool _lock;
    private float _yRot;
    private float _xRot;

    private void Update()
    {
        if (Input.GetButton("Fire1"))
        {
            Cursor.lockState = CursorLockMode.Locked;
            Cursor.visible = false;
            _lock = true;
        }

        if (Input.GetKey(KeyCode.Escape))
        {
            Cursor.lockState = CursorLockMode.None;
            Cursor.visible = true;
            _lock = false;
        }

        if (!_lock) return;
        var t = transform;

        var yLook = Input.GetAxis("Mouse X");
        var xLook = Input.GetAxis("Mouse Y");
        _yRot += yLook * rotationSpeed * Time.deltaTime * RotationScaler;
        _xRot += -xLook * rotationSpeed * Time.deltaTime * RotationScaler;
        _xRot = Mathf.Clamp(_xRot, -90, 90);
        t.rotation = Quaternion.Euler(_xRot, _yRot, 0);

        var yMove = Input.GetAxis("Vertical");
        var xMove = Input.GetAxis("Horizontal");
        var speed = Input.GetKey(KeyCode.LeftShift) || Input.GetKey(KeyCode.RightShift)
            ? flySpeed * shiftMultiplier
            : flySpeed;

        var move = Vector3.forward * yMove + Vector3.right * xMove;
        if (move.magnitude > 1) move.Normalize();
        t.Translate(move * (speed * Time.deltaTime), Space.Self);
    }
}
